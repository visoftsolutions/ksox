use std::{cmp::max, collections};

use chrono::Utc;
use futures::StreamExt;
use predicates::prelude::*;
use serde::Deserialize;
use sqlx::{postgres::PgListener, PgPool};
use thiserror::Error;
use tokio::{
    select,
    sync::{mpsc, oneshot},
};
use uuid::Uuid;

use crate::database::{
    managers,
    projections::{self},
};

#[derive(Debug, Clone, Deserialize)]
pub enum NotificationManagerEvent {
    Users,
    Valuts,
    Assets,
    SpotOrders,
    SpotTrades,
    SpotCandlesticks,
    Transfers,
    EngagementBadges,
    Deposits,
    Withdraws,
}

#[derive(Debug, Clone)]
pub enum NotificationManagerPredicateInput {
    Valuts(projections::valut::Valut),
    Assets(projections::asset::Asset),
    SpotOrders(projections::order::Order),
    SpotTrades(projections::trade::Trade),
    SpotCandlesticks(projections::candlestick::Candlestick),
    Transfers(projections::transfer::Transfer),
    Users(projections::user::User),
    EngagementBadges(projections::badge::Badge),
    Deposits(projections::deposit::Deposit),
    Withdraws(projections::withdraw::Withdraw),
}

#[derive(Debug, Clone)]
pub enum NotificationManagerOutput {
    Valuts(Vec<projections::valut::Valut>),
    Assets(Vec<projections::asset::Asset>),
    SpotOrders(Vec<projections::order::Order>),
    SpotTrades(Vec<projections::trade::Trade>),
    SpotCandlesticks(Vec<projections::candlestick::Candlestick>),
    Transfers(Vec<projections::transfer::Transfer>),
    Users(Vec<projections::user::User>),
    EngagementBadges(Vec<projections::badge::Badge>),
    Deposits(Vec<projections::deposit::Deposit>),
    Withdraws(Vec<projections::withdraw::Withdraw>),
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
            let deposits_manager = managers::deposits::DepositsManager::new(database.clone());
            let mut deposits_last_modification_at = Utc::now();
            let withdraws_manager = managers::withdraws::WithdrawsManager::new(database.clone());
            let mut withdraws_last_modification_at = Utc::now();

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
                                    let elements = valuts_manager.get_modified(valuts_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::Valuts(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::Valuts(result)).await.is_err(){
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    valuts_last_modification_at = max(
                                        valuts_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(valuts_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Assets => {
                                    let elements = assets_manager.get_modified(assets_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::Assets(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::Assets(result)).await.is_err(){
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    assets_last_modification_at = max(
                                        assets_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(assets_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::SpotOrders => {
                                    let elements = orders_manager.get_modified(orders_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotOrders(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::SpotOrders(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    orders_last_modification_at = max(
                                        orders_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(orders_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::SpotTrades => {
                                    let elements = trades_manager.get_modified(trades_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotTrades(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::SpotTrades(result)).await.is_err(){
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    trades_last_modification_at = max(
                                        trades_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(trades_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::SpotCandlesticks => {
                                    let elements = candlesticks_manager.get_modified(candlesticks_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::SpotCandlesticks(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::SpotCandlesticks(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    candlesticks_last_modification_at = max(
                                        candlesticks_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(candlesticks_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Transfers => {
                                    let elements = transfers_manager.get_modified(transfers_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::Transfers(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::Transfers(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    transfers_last_modification_at = max(
                                        transfers_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(transfers_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::EngagementBadges => {
                                    let elements = badges_manager.get_modified(badges_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::EngagementBadges(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::EngagementBadges(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    badges_last_modification_at = max(
                                        badges_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(badges_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Users => {
                                    let elements = users_manager.get_modified(users_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::Users(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::Users(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    users_last_modification_at = max(
                                        users_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(users_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Deposits => {
                                    let elements = deposits_manager.get_modified(deposits_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::Deposits(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::Deposits(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    deposits_last_modification_at = max(
                                        deposits_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(deposits_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Withdraws => {
                                    let elements = withdraws_manager.get_modified(withdraws_last_modification_at).await?;
                                    let mut set_entry_to_remove_ids = Vec::new();
                                    for set_entry in set.values() {
                                        let result: Vec<_> = elements.iter().cloned()
                                            .filter(|e| set_entry.predicate.eval(&NotificationManagerPredicateInput::Withdraws(e.clone()))).collect();
                                        if !result.is_empty() && set_entry.sender.send(NotificationManagerOutput::Withdraws(result)).await.is_err() {
                                            set_entry_to_remove_ids.push(set_entry.id);
                                        }
                                    }
                                    set_entry_to_remove_ids.into_iter().for_each(|e| {set.remove(&e);});
                                    withdraws_last_modification_at = max(
                                        withdraws_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(withdraws_last_modification_at)
                                    );
                                },
                            }

                            Ok::<(), NotificationManagerError>(())
                        }.await {
                            tracing::error!("{err}");
                        }
                    }
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
