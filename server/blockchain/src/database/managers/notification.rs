use std::{cmp::max};

use chrono::Utc;
use engagement::database::managers::notifications::NotificationManagerEvent;
use sqlx::{postgres::PgListener, PgPool};
use tokio::{select, sync::{oneshot, broadcast}};
use tokio_stream::StreamExt;

use crate::database::projections::valut::Valut;

use super::valuts::ValutsManager;

pub struct NotificationManager {}

impl NotificationManager {
    pub async fn start(
        database: PgPool,
        trigger_name: &str,
    ) -> sqlx::Result<NotificationManagerController> {
        let mut listener = PgListener::connect_with(&database).await?;
        listener.listen(trigger_name).await?;
        let mut stream = listener.into_stream();
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();
        let (broadcast, _) = broadcast::channel(100);
        let broadcast_clone = broadcast.to_owned();
        
        let join_handle = tokio::spawn(async move {
            let valuts_manager = ValutsManager::new(database.clone());
            let mut valuts_last_modification_at = Utc::now();

            loop {
                select! {
                    _ = &mut shutdown_rx => {
                        break;
                    },
                    Some(element) = stream.next() => {
                        match element {
                            Ok(value) => {
                                match serde_json::from_str::<NotificationManagerEvent>(value.payload()) {
                                    Ok(NotificationManagerEvent::SpotValutsChanged) => {
                                        match valuts_manager.get_modified(valuts_last_modification_at).await {
                                            Ok(elements) => {
                                                if broadcast.receiver_count() != 0 {
                                                    if let Err(err) = broadcast.send(elements.clone()) {
                                                        tracing::error!("Error: {}", err);
                                                    }
                                                }
                                                valuts_last_modification_at = max(
                                                    valuts_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(valuts_last_modification_at)
                                                );
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotAssetsChanged) => {},
                                    Ok(NotificationManagerEvent::SpotOrdersChanged) => {},
                                    Ok(NotificationManagerEvent::SpotTradesChanged) => {},
                                    Ok(NotificationManagerEvent::SpotCandlesticksChanged) => {},
                                    Ok(NotificationManagerEvent::TransfersChanged) => {},
                                    Ok(NotificationManagerEvent::EngagementBadgesChanged) => {},
                                    Ok(NotificationManagerEvent::UsersChanged) => {},
                                    Ok(NotificationManagerEvent::DepositsChanged) => {},
                                    Ok(NotificationManagerEvent::WithdrawsChanged) => {},
                                    Err(err) => {
                                        tracing::error!("Error: {}", err);
                                    }
                                }
                            },
                            Err(err) => {
                                tracing::error!("Error: {}", err);
                            }
                        }
                    },
                }
            }
        });

        Ok(NotificationManagerController {
            shutdown_tx,
            broadcast: broadcast_clone,
            join_handle,
        })
    }
}

#[derive(Debug)]
pub struct NotificationManagerController {
    shutdown_tx: oneshot::Sender<()>,
    broadcast: broadcast::Sender<Vec<Valut>>,
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

    pub fn is_finished(&self) -> bool {
        self.join_handle.is_finished()
    }

    pub async fn subscribe_to(&self) -> broadcast::Receiver<Vec<Valut>> {
        self.broadcast.subscribe()
    }
}
