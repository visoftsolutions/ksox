use std::{cmp::max, collections::HashSet};

use badges::{trade, transfer, valut, BadgeEval, BadgeMetric};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{postgres::PgListener, PgPool};
use thiserror::Error;
use tokio::{select, sync::oneshot};
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    database::{
        managers::{self},
        projections::badge::{BadgeName, TradeBadge, TransferBadge, ValutBadge},
    },
    engagement_engine::badges,
};

#[derive(Debug, Clone, Deserialize)]
pub enum NotificationManagerEvent {
    Users,
    Valuts,
    SpotTrades,
    Transfers,
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
        let (shutdown_tx, mut shutdown_rx) = oneshot::channel();

        let join_handle = tokio::spawn(async move {
            let valuts_manager = managers::valuts::ValutsManager::new(database.clone());
            let mut valuts_last_modification_at = Utc::now();
            let trades_manager = managers::trades::TradesManager::new(database.clone());
            let mut trades_last_modification_at = Utc::now();
            let transfers_manager = managers::transfers::TransfersManager::new(database.clone());
            let mut transfers_last_modification_at = Utc::now();
            let badges_manager = managers::badges::BadgesManager::new(database.clone());
            let users_manager = managers::users::UsersManager::new(database.clone());
            let mut users_last_modification_at = Utc::now();

            loop {
                select! {
                    _ = &mut shutdown_rx => {
                        break;
                    },
                    Some(element) = stream.next() => {
                        if let Err(err) = async {
                            match serde_json::from_str::<NotificationManagerEvent>(element?.payload())? {
                                NotificationManagerEvent::Valuts => {
                                    let elements = valuts_manager.get_modified(valuts_last_modification_at).await?;
                                    let users: HashSet<Uuid> = elements.iter().cloned().map(|e| e.user_id).collect();
                                    for user_id in users {
                                        let current_badges = badges_manager.get_for_user_id(user_id).await.map(|e| e.into_iter().map(|e| e.badge_name).collect::<HashSet<BadgeName>>())?;
                                        let mut metric = (ValutBadge::metric())(&valut::MetricInput{valuts_manager: &valuts_manager, user_id}).await?;
                                        let badges_to_assign = ValutBadge::eval_recived(&mut metric).into_iter().map(BadgeName::ValutBadge).collect::<HashSet<BadgeName>>().difference(&current_badges).cloned().collect::<HashSet<BadgeName>>();
                                        for badge in badges_to_assign {
                                            badges_manager.assign_badge(user_id, badge).await?;
                                        }
                                    }
                                    valuts_last_modification_at = max(
                                        valuts_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(valuts_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::SpotTrades => {
                                    let elements = trades_manager.get_modified(trades_last_modification_at).await?;
                                    let users: HashSet<Uuid> = elements.iter().cloned().map(|e| e.maker_id).collect();
                                    for user_id in users {
                                        let current_badges = badges_manager.get_for_user_id(user_id).await.map(|e| e.into_iter().map(|e| e.badge_name).collect::<HashSet<BadgeName>>())?;
                                        let mut metric = (TradeBadge::metric())(&trade::MetricInput{trades_manager: &trades_manager, user_id}).await?;
                                        let badges_to_assign = TradeBadge::eval_recived(&mut metric).into_iter().map(BadgeName::TradeBadge).collect::<HashSet<BadgeName>>().difference(&current_badges).cloned().collect::<HashSet<BadgeName>>();
                                        for badge in badges_to_assign {
                                            badges_manager.assign_badge(user_id, badge).await?;
                                        }
                                    }
                                    trades_last_modification_at = max(
                                        valuts_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(trades_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Transfers => {
                                    let elements = trades_manager.get_modified(transfers_last_modification_at).await?;
                                    let users: HashSet<Uuid> = elements.iter().cloned().map(|e| e.maker_id).collect();
                                    for user_id in users {
                                        let current_badges = badges_manager.get_for_user_id(user_id).await.map(|e| e.into_iter().map(|e| e.badge_name).collect::<HashSet<BadgeName>>())?;
                                        let mut metric = (TransferBadge::metric())(&transfer::MetricInput{transfers_manager: &transfers_manager, user_id}).await?;
                                        let badges_to_assign = TransferBadge::eval_recived(&mut metric).into_iter().map(BadgeName::TransferBadge).collect::<HashSet<BadgeName>>().difference(&current_badges).cloned().collect::<HashSet<BadgeName>>();
                                        for badge in badges_to_assign {
                                            badges_manager.assign_badge(user_id, badge).await?;
                                        }
                                    }
                                    transfers_last_modification_at = max(
                                        valuts_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(transfers_last_modification_at)
                                    );
                                },
                                NotificationManagerEvent::Users => {
                                    let elements = users_manager.get_modified(users_last_modification_at).await?;
                                    users_last_modification_at = max(
                                        valuts_last_modification_at,
                                        elements.into_iter().map(|e| e.last_modification_at).max().unwrap_or(users_last_modification_at)
                                    );
                                }
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
}
