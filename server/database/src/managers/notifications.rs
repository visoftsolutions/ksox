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

pub struct NotificationManagerEntry {
    id: uuid::Uuid,
    predicate: Box<dyn Predicate<NotificationManagerPredicateInput> + Send + Sync>,
    sender: mpsc::Sender<NotificationManagerOutput>,
}
impl NotificationManagerEntry {
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
            let valuts_manager = managers::spot::valuts::ValutsManager::new(database.clone());
            let mut valuts_last_modification_at = Utc::now();
            let assets_manager = managers::spot::assets::AssetsManager::new(database.clone());
            let mut assets_last_modification_at = Utc::now();
            let orders_manager = managers::spot::orders::OrdersManager::new(database.clone());
            let mut orders_last_modification_at = Utc::now();
            let trades_manager = managers::spot::trades::TradesManager::new(database.clone());
            let mut trades_last_modification_at = Utc::now();
            let candlesticks_manager =
                managers::spot::candlesticks::CandlesticksManager::new(database.clone());
            let mut candlesticks_last_modification_at = Utc::now();

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
                        tracing::error!("{:?}", element);
                        match element {
                            Ok(value) => {
                                match serde_json::from_str::<NotificationManagerEvent>(&value.payload()) {
                                    Ok(NotificationManagerEvent::SpotValutsChanged) => {
                                        match valuts_manager.get_modified(valuts_last_modification_at).await {
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
                                                                tracing::info!("{}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                                for valut in valuts.iter() {
                                                    if valut.last_modification_at > valuts_last_modification_at {
                                                        valuts_last_modification_at = valut.last_modification_at;
                                                    }
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotAssetsChanged) => {
                                        match assets_manager.get_modified(assets_last_modification_at).await {
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
                                                                tracing::info!("{}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                                for asset in assets.iter() {
                                                    if asset.last_modification_at > assets_last_modification_at {
                                                        assets_last_modification_at = asset.last_modification_at;
                                                    }
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotOrdersChanged) => {
                                        match orders_manager.get_modified(orders_last_modification_at).await {
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
                                                                tracing::info!("{}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                                for order in orders.iter() {
                                                    if order.last_modification_at > orders_last_modification_at {
                                                        orders_last_modification_at = order.last_modification_at;
                                                    }
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotTradesChanged) => {
                                        match trades_manager.get_modified(trades_last_modification_at).await {
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
                                                                tracing::info!("{}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                                for trade in trades.iter() {
                                                    if trade.last_modification_at > trades_last_modification_at {
                                                        trades_last_modification_at = trade.last_modification_at;
                                                    }
                                                }
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotCandlesticksChanged) => {
                                        match candlesticks_manager.get_modified(candlesticks_last_modification_at).await {
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
                                                                tracing::info!("{}", err);
                                                            }
                                                        }
                                                    }
                                                }
                                                for set_entry_to_remove_id in set_entry_to_remove_ids {
                                                    set.remove(&set_entry_to_remove_id);
                                                }
                                                for candlestick in candlesticks.iter() {
                                                    if candlestick.last_modification_at > candlesticks_last_modification_at {
                                                        candlesticks_last_modification_at = candlestick.last_modification_at;
                                                    }
                                                }
                                            },
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

#[derive(Debug)]
pub struct NotificationManagerController {
    tx: mpsc::Sender<NotificationManagerEntry>,
    shutdown_tx: oneshot::Sender<()>,
    join_handle: tokio::task::JoinHandle<()>,
}
impl NotificationManagerController {
    pub async fn shutdown(self) -> Result<(), tokio::task::JoinError> {
        if let Err(_) = self.shutdown_tx.send(()) {
            tracing::error!("Error: shutdown");
        }
        self.join_handle.await?;
        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.join_handle.is_finished()
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
        predicate: Box<dyn Predicate<NotificationManagerPredicateInput> + Send + Sync>,
    ) -> Result<
        mpsc::Receiver<NotificationManagerOutput>,
        mpsc::error::SendError<NotificationManagerEntry>,
    > {
        let (tx, rx) = mpsc::channel(100);
        let set_entry = NotificationManagerEntry::new(predicate, tx);
        if let Err(err) = self.tx.send(set_entry).await {
            tracing::error!("Error: {}", err);
            Err(err)
        } else {
            Ok(rx)
        }
    }
}
