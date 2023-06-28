use std::{cmp::max, collections};

use chrono::Utc;
use engagement::database::managers::notifications::NotificationManagerEvent;
use futures::StreamExt;
use predicates::prelude::*;
use sqlx::{postgres::PgListener, PgPool};
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::database::{
    managers,
    projections::{
        self, asset::Asset, badge::Badge, candlestick::Candlestick, order::Order, trade::Trade,
        transfer::Transfer, user::User, valut::Valut,
    },
};

#[derive(Debug, Clone)]
pub enum NotificationManagerPredicateInput {
    SpotValutsChanged(projections::valut::Valut),
    SpotAssetsChanged(projections::asset::Asset),
    SpotOrdersChanged(projections::order::Order),
    SpotTradesChanged(projections::trade::Trade),
    SpotCandlesticksChanged(projections::candlestick::Candlestick),
    TransfersChanged(projections::transfer::Transfer),
    UsersChanged(projections::user::User),
    EngagementBadgesChanged(projections::badge::Badge),
    DepositsChanged(projections::deposit::Deposit),
    WithdrawsChanged(projections::withdraw::Withdraw),
}

#[derive(Debug, Clone)]
pub enum NotificationManagerOutput {
    SpotValutsChanged(Vec<projections::valut::Valut>),
    SpotAssetsChanged(Vec<projections::asset::Asset>),
    SpotOrdersChanged(Vec<projections::order::Order>),
    SpotTradesChanged(Vec<projections::trade::Trade>),
    SpotCandlesticksChanged(Vec<projections::candlestick::Candlestick>),
    TransfersChanged(Vec<projections::transfer::Transfer>),
    UsersChanged(Vec<projections::user::User>),
    EngagementBadgesChanged(Vec<projections::badge::Badge>),
    DepositsChanged(Vec<projections::deposit::Deposit>),
    WithdrawsChanged(Vec<projections::withdraw::Withdraw>),
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
            id: Uuid::new_v4(),
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
            let valuts_manager = managers::valuts::ValutsManager::new(database.clone());
            let mut valuts_last_modification_at = Utc::now();
            let assets_manager = managers::assets::AssetsManager::new(database.clone());
            let mut assets_last_modification_at = Utc::now();
            let orders_manager = managers::orders::OrdersManager::new(database.clone());
            let mut orders_last_modification_at = Utc::now();
            let trades_manager = managers::trades::TradesManager::new(database.clone());
            let mut trades_last_modification_at = Utc::now();
            let candlesticks_manager =
                managers::candlesticks::CandlesticksManager::new(database.clone());
            let mut candlesticks_last_modification_at = Utc::now();
            let transfers_manager = managers::transfers::TransfersManager::new(database.clone());
            let mut transfers_last_modification_at = Utc::now();
            let badges_manager = managers::badges::BadgesManager::new(database.clone());
            let mut badges_last_modification_at = Utc::now();
            let users_manager = managers::users::UsersManager::new(database.clone());
            let mut users_last_modification_at = Utc::now();

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
                                match serde_json::from_str::<NotificationManagerEvent>(value.payload()) {
                                    Ok(NotificationManagerEvent::SpotValutsChanged) => {
                                        match valuts_manager.get_modified(valuts_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Valut> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotValutsChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::SpotValutsChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

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
                                    Ok(NotificationManagerEvent::SpotAssetsChanged) => {
                                        match assets_manager.get_modified(assets_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Asset> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotAssetsChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::SpotAssetsChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                assets_last_modification_at = max(
                                                    assets_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(assets_last_modification_at)
                                                );
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotOrdersChanged) => {
                                        match orders_manager.get_modified(orders_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Order> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotOrdersChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::SpotOrdersChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                orders_last_modification_at = max(
                                                    orders_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(orders_last_modification_at)
                                                );
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotTradesChanged) => {
                                        match trades_manager.get_modified(trades_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Trade> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotTradesChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::SpotTradesChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                trades_last_modification_at = max(
                                                    trades_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(trades_last_modification_at)
                                                );
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::SpotCandlesticksChanged) => {
                                        match candlesticks_manager.get_modified(candlesticks_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Candlestick> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotCandlesticksChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::SpotCandlesticksChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                candlesticks_last_modification_at = max(
                                                    candlesticks_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(candlesticks_last_modification_at)
                                                );

                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::TransfersChanged) => {
                                        match transfers_manager.get_modified(transfers_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Transfer> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::TransfersChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::TransfersChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                transfers_last_modification_at = max(
                                                    transfers_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(transfers_last_modification_at)
                                                );

                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::EngagementBadgesChanged) => {
                                        match badges_manager.get_modified(badges_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<Badge> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::EngagementBadgesChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::EngagementBadgesChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                badges_last_modification_at = max(
                                                    badges_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(badges_last_modification_at)
                                                );

                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    }
                                    Ok(NotificationManagerEvent::UsersChanged) => {
                                        match users_manager.get_modified(users_last_modification_at).await {
                                            Ok(elements) => {
                                                let mut set_entry_to_remove_ids = Vec::new();
                                                for set_entry in set.values() {
                                                    let result: Vec<User> = elements.iter().cloned()
                                                        .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::UsersChanged(e.clone()))).collect();

                                                    if !result.is_empty() {
                                                        if let Err(err) = set_entry.sender.send(NotificationManagerOutput::UsersChanged(result)).await {
                                                            set_entry_to_remove_ids.push(set_entry.id);
                                                            tracing::info!("{}", err);
                                                        }
                                                    }
                                                }
                                                set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});

                                                users_last_modification_at = max(
                                                    users_last_modification_at,
                                                    elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(users_last_modification_at)
                                                );
                                            },
                                            Err(err) => {
                                                tracing::error!("Error: {}", err);
                                            }
                                        }
                                    },
                                    Ok(NotificationManagerEvent::DepositsChanged) => {
                                        
                                    },
                                    Ok(NotificationManagerEvent::WithdrawsChanged) => {
                                        
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
        if self.shutdown_tx.send(()).is_err() {
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
