use std::{cmp::max, collections};

use chrono::Utc;
use serde::Deserialize;
use sqlx::{postgres::PgListener, PgPool};
use thiserror::Error;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use tokio_stream::StreamExt;
use uuid::Uuid;

use super::valuts::ValutsManager;
use crate::database::projections::{self, valut::ValutFinite};

#[derive(Debug, Clone, Deserialize)]
pub enum NotificationManagerEvent {
    Valuts,
}

#[derive(Debug, Clone)]
pub enum NotificationManagerPredicateInput {
    Valuts(projections::valut::ValutFinite),
}

#[derive(Debug, Clone)]
pub enum NotificationManagerOutput {
    Valuts(Vec<projections::valut::ValutFinite>),
}

#[derive(Debug)]
pub struct NotificationManagerEntry {
    id: uuid::Uuid,
    sender: mpsc::Sender<NotificationManagerOutput>,
}
impl NotificationManagerEntry {
    pub fn new(sender: mpsc::Sender<NotificationManagerOutput>) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
        }
    }
}
impl PartialEq for NotificationManagerEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for NotificationManagerEntry {}
impl std::hash::Hash for NotificationManagerEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct NotificationManager {}

impl NotificationManager {
    pub async fn start(
        database: PgPool,
        trigger_name: &str,
    ) -> sqlx::Result<NotificationManagerController> {
        let mut listener = PgListener::connect_with(&database).await?;
        listener.listen(trigger_name).await?;
        let mut stream = listener.into_stream();
        let mut set = collections::HashMap::<Uuid, NotificationManagerEntry>::new();
        let (tx, mut rx) = mpsc::channel::<NotificationManagerEntry>(100);
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        let join_handle = tokio::spawn(async move {
            let valuts_manager = ValutsManager::new(database.clone());
            let mut valuts_last_modification_at = Utc::now();

            loop {
                select! {
                    _ = &mut shutdown_rx => {
                        break;
                    },
                    set_entry = rx.recv() => {
                        match set_entry {
                            Some(set_entry) => {
                                set.insert(set_entry.id, set_entry);
                            },
                            None => {
                                break;
                            }
                        }
                    },
                    Some(element) = stream.next() => {
                        if let Err(err) = async {
                            match serde_json::from_str::<NotificationManagerEvent>(element?.payload())? {
                                NotificationManagerEvent::Valuts => {
                                    // dont want to update perpetual account
                                    let elements: Vec<ValutFinite> = valuts_manager.get_modified(valuts_last_modification_at).await?.into_iter().filter_map(ValutFinite::checked_from).collect();
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        if set_entry.sender.send(NotificationManagerOutput::Valuts(elements.clone())).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                    valuts_last_modification_at = max(
                                        valuts_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(valuts_last_modification_at)
                                    );
                                },
                            }

                            Ok::<(), NotificationManagerError>(())
                        }.await {
                            tracing::error!("{err}");
                        }
                    },
                }
            }
        });

        Ok(NotificationManagerController {
            tx,
            shutdown_tx,
            join_handle,
        })
    }
}

#[derive(Error, Debug)]
pub enum NotificationManagerError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

#[derive(Debug)]
pub struct NotificationManagerController {
    tx: mpsc::Sender<NotificationManagerEntry>,
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<()>,
}
impl NotificationManagerController {
    pub async fn shutdown(self) -> Result<(), tokio::task::JoinError> {
        if self.shutdown_tx.send(()).is_err() {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await?;
        Ok(())
    }

    pub fn get_subscriber(&self) -> NotificationManagerSubscriber {
        NotificationManagerSubscriber {
            tx: self.tx.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotificationManagerSubscriber {
    tx: mpsc::Sender<NotificationManagerEntry>,
}
impl NotificationManagerSubscriber {
    pub async fn subscribe_to(
        &self,
    ) -> Result<
        mpsc::Receiver<NotificationManagerOutput>,
        mpsc::error::SendError<NotificationManagerEntry>,
    > {
        let (tx, rx) = mpsc::channel(100);
        let set_entry = NotificationManagerEntry::new(tx);
        if let Err(err) = self.tx.send(set_entry).await {
            tracing::error!("Error: {}", err);
            Err(err)
        } else {
            Ok(rx)
        }
    }
}
