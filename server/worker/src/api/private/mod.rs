pub mod active;
pub mod balance;
pub mod cancel;
pub mod deposits;
pub mod engagement;
pub mod orders;
pub mod submit;
pub mod trades;
pub mod transfer;
pub mod transfers;
pub mod user;
pub mod withdraw;
pub mod withdraws;

use axum::{
    extract::State,
    response::{sse::Event, Sse},
    routing::{delete, get, post},
    Router,
};
use chrono::{DateTime, Utc};
use fraction::{num_traits::Inv, Fraction};
use uuid::Uuid;

use std::{convert::Infallible, time::Duration};

use futures::{stream, Stream};
use serde::Serialize;
use tokio_stream::StreamExt;

use super::{auth::models::UserId, Direction};
use crate::{
    database::projections::{order::Order, trade::Trade},
    models::AppState,
};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/sse", get(sse))
        .route("/cancel", delete(cancel::root))
        .route("/submit", post(submit::root))
        .route("/transfer", post(transfer::root))
        .route("/withdraw", post(withdraw::root))
        .with_state(app_state.clone())
        .nest("/user", user::router(app_state))
        .nest("/active", active::router(app_state))
        .nest("/balance", balance::router(app_state))
        .nest("/orders", orders::router(app_state))
        .nest("/trades", trades::router(app_state))
        .nest("/transfers", transfers::router(app_state))
        .nest("/engagement", engagement::router(app_state))
        .nest("/deposits", deposits::router(app_state))
        .nest("/withdraws", withdraws::router(app_state))
}

pub async fn root(State(_state): State<AppState>, user_id: UserId) -> String {
    format!("Hello, World private! - {}", *user_id)
}

pub async fn sse(
    State(_state): State<AppState>,
    user_id: UserId,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(move || {
        Event::default().data(format!(
            "Hello, World private! - {}, time: {}",
            *user_id,
            Utc::now()
        ))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

#[derive(Serialize)]
pub struct ResponseOrder {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub quote_asset_volume: Fraction,
    pub quote_asset_volume_left: Fraction,
    pub maker_fee: Fraction,
    pub direction: Direction,
}

impl ResponseOrder {
    pub fn from(value: Order) -> Self {
        if !value.maker_presentation {
            Self {
                id: value.id,
                created_at: value.created_at,
                is_active: value.is_active,
                quote_asset_id: value.quote_asset_id,
                base_asset_id: value.base_asset_id,
                price: value.price,
                quote_asset_volume: value.quote_asset_volume,
                quote_asset_volume_left: value.quote_asset_volume_left,
                maker_fee: value.maker_fee,
                direction: Direction::Buy,
            }
        } else {
            Self {
                id: value.id,
                created_at: value.created_at,
                is_active: value.is_active,
                quote_asset_id: value.base_asset_id,
                base_asset_id: value.quote_asset_id,
                price: value.price.clone().inv(),
                quote_asset_volume: value.quote_asset_volume / value.price.clone(),
                quote_asset_volume_left: value.quote_asset_volume_left / value.price,
                maker_fee: value.maker_fee,
                direction: Direction::Sell,
            }
        }
    }
}

#[derive(Serialize)]
pub struct ResponseTrade {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub price: Fraction,
    pub taker_quote_volume: Fraction,
    pub maker_quote_volume: Fraction,
    pub direction: Direction,
}

impl ResponseTrade {
    pub fn from(value: Trade, user_id: Uuid) -> Self {
        if value.taker_id == user_id {
            if !value.taker_presentation {
                Self {
                    id: value.id,
                    created_at: value.created_at,
                    quote_asset_id: value.quote_asset_id,
                    base_asset_id: value.base_asset_id,
                    price: value.price,
                    taker_quote_volume: value.taker_quote_volume,
                    maker_quote_volume: value.maker_quote_volume,
                    direction: Direction::Buy,
                }
            } else {
                Self {
                    id: value.id,
                    created_at: value.created_at,
                    quote_asset_id: value.base_asset_id,
                    base_asset_id: value.quote_asset_id,
                    price: value.price.inv(),
                    taker_quote_volume: value.maker_quote_volume,
                    maker_quote_volume: value.taker_quote_volume,
                    direction: Direction::Sell,
                }
            }
        } else if !value.maker_presentation {
            Self {
                id: value.id,
                created_at: value.created_at,
                quote_asset_id: value.base_asset_id,
                base_asset_id: value.quote_asset_id,
                price: value.price.inv(),
                taker_quote_volume: value.maker_quote_volume,
                maker_quote_volume: value.taker_quote_volume,
                direction: Direction::Buy,
            }
        } else {
            Self {
                id: value.id,
                created_at: value.created_at,
                quote_asset_id: value.quote_asset_id,
                base_asset_id: value.base_asset_id,
                price: value.price,
                taker_quote_volume: value.taker_quote_volume,
                maker_quote_volume: value.maker_quote_volume,
                direction: Direction::Sell,
            }
        }
    }
}
