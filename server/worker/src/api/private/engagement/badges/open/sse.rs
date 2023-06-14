use std::{
    io::{Error, ErrorKind},
    time::Duration,
};

use axum::{
    extract::{Query, State},
    response::sse::{Event, Sse},
};
use engagement::{
    database::{
        managers::{trades::TradesManager, transfers::TransfersManager, valuts::ValutsManager},
        projections::badge::{
            BadgeFamily, MakerBadge, TakerBadge, TradeBadge, TransferBadge, ValutBadge,
        },
    },
    engagement_engine::badges::{
        maker, taker, trade, transfer, valut, BadgeEval, BadgeMetric, BadgeValue,
    },
};
use fraction::Fraction;
use futures::stream::Stream;
use num_bigint::BigInt;
use tokio_stream::StreamExt;

use super::{Request, Response};
use crate::{api::auth::models::UserId, models::AppState};

pub async fn root(
    State(state): State<AppState>,
    user_id: UserId,
    Query(request): Query<Request>,
) -> Sse<impl Stream<Item = Result<Event, std::io::Error>>> {
    let stream = async_stream::stream! {
        match request.badge_family {
            BadgeFamily::ValutBadge => {
                let mut stream = state.valuts_notification_manager.subscribe_to_user(*user_id).await
                    .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
                let manager = ValutsManager::new(state.database);
                while stream.next().await.is_some() {
                    let mut metric = (ValutBadge::metric())(&valut::MetricInput {
                        valuts_manager: &manager,
                        user_id: *user_id,
                    })
                    .await.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

                    yield Event::default().json_data(
                        ValutBadge::eval_open(&mut metric).map(|f| f.to_value()).map(|f|
                            Response {
                                name: f.name.to_string(),
                                description: f.description.to_string(),
                                progress: Fraction::from_raw((
                                    BigInt::from(metric),
                                    BigInt::from(f.value),
                                ))
                                .unwrap_or_default(),
                            }
                        )
                    ).map_err(Error::from);
                };
            }
            BadgeFamily::TradeBadge => {
                let mut stream = state.trades_notification_manager.subscribe_to_user(*user_id).await
                    .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
                let manager = TradesManager::new(state.database);
                while stream.next().await.is_some() {
                    let mut metric = (TradeBadge::metric())(&trade::MetricInput {
                        trades_manager: &manager,
                        user_id: *user_id,
                    })
                    .await.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

                    yield Event::default().json_data(
                        TradeBadge::eval_open(&mut metric).map(|f| f.to_value()).map(|f|
                            Response {
                                name: f.name.to_string(),
                                description: f.description.to_string(),
                                progress: Fraction::from_raw((
                                    BigInt::from(metric),
                                    BigInt::from(f.value),
                                ))
                                .unwrap_or_default(),
                            }
                        )
                    ).map_err(Error::from);
                };
            }
            BadgeFamily::TransferBadge => {
                let mut stream = state.transfers_notification_manager.subscribe_to_user(*user_id).await
                    .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
                let manager = TransfersManager::new(state.database);
                while stream.next().await.is_some() {
                    let mut metric = (TransferBadge::metric())(&transfer::MetricInput {
                        transfers_manager: &manager,
                        user_id: *user_id,
                    })
                    .await.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

                    yield Event::default().json_data(
                        TransferBadge::eval_open(&mut metric).map(|f| f.to_value()).map(|f|
                            Response {
                                name: f.name.to_string(),
                                description: f.description.to_string(),
                                progress: Fraction::from_raw((
                                    BigInt::from(metric),
                                    BigInt::from(f.value),
                                ))
                                .unwrap_or_default(),
                            }
                        )
                    ).map_err(Error::from);
                };
            }
            BadgeFamily::TakerBadge => {
                let mut stream = state.trades_notification_manager.subscribe_to_user(*user_id).await
                    .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
                let manager = TradesManager::new(state.database);
                while stream.next().await.is_some() {
                    let mut metric = (TakerBadge::metric())(&taker::MetricInput {
                        trades_manager: &manager,
                        user_id: *user_id,
                    })
                    .await.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

                    yield Event::default().json_data(
                        TakerBadge::eval_open(&mut metric).map(|f| f.to_value()).map(|f|
                            Response {
                                name: f.name.to_string(),
                                description: f.description.to_string(),
                                progress: Fraction::from_raw((
                                    BigInt::from(metric),
                                    BigInt::from(f.value),
                                ))
                                .unwrap_or_default(),
                            }
                        )
                    ).map_err(Error::from);
                };
            }
            BadgeFamily::MakerBadge => {
                let mut stream = state.trades_notification_manager.subscribe_to_user(*user_id).await
                    .map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;
                let manager = TradesManager::new(state.database);
                while stream.next().await.is_some() {
                    let mut metric = (MakerBadge::metric())(&maker::MetricInput {
                        trades_manager: &manager,
                        user_id: *user_id,
                    })
                    .await.map_err(|err| Error::new(ErrorKind::BrokenPipe, err))?;

                    yield Event::default().json_data(
                        MakerBadge::eval_open(&mut metric).map(|f| f.to_value()).map(|f|
                            Response {
                                name: f.name.to_string(),
                                description: f.description.to_string(),
                                progress: Fraction::from_raw((
                                    BigInt::from(metric),
                                    BigInt::from(f.value),
                                ))
                                .unwrap_or_default(),
                            }
                        )
                    ).map_err(Error::from);
                };
            }
        }

    };

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive-text"),
    )
}
