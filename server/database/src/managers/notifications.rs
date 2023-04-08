use std::collections;

use chrono::Utc;
use futures::StreamExt;
use predicates::prelude::*;
use serde::Deserialize;
use sqlx::{postgres::PgListener, PgPool};
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::{managers, projections, traits::GetModified};

#[derive(Debug, Clone, Deserialize)]
pub enum NotificationManagerEvent {
    SpotValutsChanged,
    SpotAssetsChanged,
    SpotOrdersChanged,
    SpotTradesChanged,
    SpotCandlesticksChanged,
}

#[derive(Debug, Clone)]
pub enum NotificationManagerPredicateInput {
    SpotValutsChanged(projections::spot::valut::Valut),
    SpotAssetsChanged(projections::spot::asset::Asset),
    SpotOrdersChanged(projections::spot::order::Order),
    SpotTradesChanged(projections::spot::trade::Trade),
    SpotCandlesticksChanged(projections::spot::candlestick::Candlestick),
}

#[derive(Debug, Clone)]
pub enum NotificationManagerOutput {
    SpotValutsChanged(Vec<projections::spot::valut::Valut>),
    SpotAssetsChanged(Vec<projections::spot::asset::Asset>),
    SpotOrdersChanged(Vec<projections::spot::order::Order>),
    SpotTradesChanged(Vec<projections::spot::trade::Trade>),
    SpotCandlesticksChanged(Vec<projections::spot::candlestick::Candlestick>),
}

pub struct SetEntry {
    id: uuid::Uuid,
    predicate: Box<dyn Predicate<NotificationManagerPredicateInput> + Send + Sync>,
    sender: mpsc::Sender<NotificationManagerOutput>,
}
impl SetEntry {
    pub fn new(
        predicate: Box<dyn Predicate<NotificationManagerPredicateInput> + Send + Sync>,
        sender: mpsc::Sender<NotificationManagerOutput>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            predicate,
            sender,
        }
    }
}
impl PartialEq for SetEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for SetEntry {}
impl std::hash::Hash for SetEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct NotificationManager {
    database: PgPool,
    tx: mpsc::Sender<SetEntry>,
}

impl NotificationManager {
    pub async fn start(
        database: PgPool,
        trigger_name: &str,
    ) -> sqlx::Result<NotificationManagerController> {
        let mut listener = PgListener::connect_with(&database).await?;
        listener.listen(trigger_name).await?;
        let mut stream = listener.into_stream();

        let mut set = collections::HashMap::<Uuid, SetEntry>::new();

        let (tx, mut rx) = mpsc::channel::<SetEntry>(100);
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        let join_handle = tokio::spawn(async move {
            let valuts_manager = managers::spot::valuts::ValutsManager::new(database.clone());
            let assets_manager = managers::spot::assets::AssetsManager::new(database.clone());
            let orders_manager = managers::spot::orders::OrdersManager::new(database.clone());
            let trades_manager = managers::spot::trades::TradesManager::new(database.clone());
            let candlesticks_manager =
                managers::spot::candlesticks::CandlesticksManager::new(database.clone());

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
                        match element {
                            Ok(value) => {
                                match serde_json::from_str::<NotificationManagerEvent>(&value.payload()) {
                                    Ok(NotificationManagerEvent::SpotValutsChanged) => {
                                        match valuts_manager.get_modified(Utc::now()).await {
                                            Ok(valuts) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let mut result = Vec::new();
                                                    for valut in valuts.iter() {
                                                        if set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotValutsChanged(valut.clone())) {
                                                            result.push(valut.clone());
                                                        }
                                                    }
                                                    if !result.is_empty() {
                                                        match set_entry.sender.send(NotificationManagerOutput::SpotValutsChanged(result)).await {
                                                            Ok(_) => {},
                                                            Err(err) => {
                                                                set_entry_to_remove_ids.push(set_entry.id);
                                                                tracing::error!("Error: {}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotAssetsChanged) => {
                                        match assets_manager.get_modified(Utc::now()).await {
                                            Ok(assets) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let mut result = Vec::new();
                                                    for asset in assets.iter() {
                                                        if set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotAssetsChanged(asset.clone())) {
                                                            result.push(asset.clone());
                                                        }
                                                    }
                                                    if !result.is_empty() {
                                                        match set_entry.sender.send(NotificationManagerOutput::SpotAssetsChanged(result)).await {
                                                            Ok(_) => {},
                                                            Err(err) => {
                                                                set_entry_to_remove_ids.push(set_entry.id);
                                                                tracing::error!("Error: {}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }

                                    },
                                    Ok(NotificationManagerEvent::SpotOrdersChanged) => {
                                        match orders_manager.get_modified(Utc::now()).await {
                                            Ok(orders) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let mut result = Vec::new();
                                                    for order in orders.iter() {
                                                        if set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotOrdersChanged(order.clone())) {
                                                            result.push(order.clone());
                                                        }
                                                    }
                                                    if !result.is_empty() {
                                                        match set_entry.sender.send(NotificationManagerOutput::SpotOrdersChanged(result)).await {
                                                            Ok(_) => {},
                                                            Err(err) => {
                                                                set_entry_to_remove_ids.push(set_entry.id);
                                                                tracing::error!("Error: {}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }

                                    },
                                    Ok(NotificationManagerEvent::SpotTradesChanged) => {
                                        match trades_manager.get_modified(Utc::now()).await {
                                            Ok(trades) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let mut result = Vec::new();
                                                    for trade in trades.iter() {
                                                        if set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotTradesChanged(trade.clone())) {
                                                            result.push(trade.clone());
                                                        }
                                                    }
                                                    if !result.is_empty() {
                                                        match set_entry.sender.send(NotificationManagerOutput::SpotTradesChanged(result)).await {
                                                            Ok(_) => {},
                                                            Err(err) => {
                                                                set_entry_to_remove_ids.push(set_entry.id);
                                                                tracing::error!("Error: {}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }

                                    },
                                    Ok(NotificationManagerEvent::SpotCandlesticksChanged) => {
                                        match candlesticks_manager.get_modified(Utc::now()).await {
                                            Ok(candlesticks) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let mut result = Vec::new();
                                                    for candlestick in candlesticks.iter() {
                                                        if set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotCandlesticksChanged(candlestick.clone())) {
                                                            result.push(candlestick.clone());
                                                        }
                                                    }
                                                    if !result.is_empty() {
                                                        match set_entry.sender.send(NotificationManagerOutput::SpotCandlesticksChanged(result)).await {
                                                            Ok(_) => {},
                                                            Err(err) => {
                                                                set_entry_to_remove_ids.push(set_entry.id);
                                                                tracing::error!("Error: {}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }

                                    },
                                    Err(err) => {
                                        tracing::error!("Error: {}", err);
                                        break;
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
            tx,
            shutdown_tx,
            join_handle,
        })
    }
}

pub struct NotificationManagerController {
    tx: mpsc::Sender<SetEntry>,
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<()>,
}
impl NotificationManagerController {
    pub async fn shutdown(self) -> Result<(), tokio::task::JoinError> {
        self.shutdown_tx.send(());
        self.join_handle.await?;
        Ok(())
    }

    pub async fn subscribe_to(
        &self,
        predicate: Box<dyn Predicate<NotificationManagerPredicateInput> + Send + Sync>,
    ) -> Result<mpsc::Receiver<NotificationManagerOutput>, mpsc::error::SendError<SetEntry>> {
        let (tx, rx) = mpsc::channel(100);
        let set_entry = SetEntry::new(predicate, tx);
        self.tx.send(set_entry).await?;
        Ok(rx)
    }
}
