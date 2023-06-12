use std::{error::Error, fmt::Display, io, str::FromStr};

use axum::{extract::Query, routing::get, Json, Router};
use engagement::{
    database::projections::badge::{
        BadgeFamily, EnumValue, MakerBadge, TakerBadge, TradeBadge, TransferBadge, ValutBadge,
    },
    engagement_engine::badges::BadgeValue,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

use crate::{api::AppError, models::AppState};

pub fn router(app_state: &AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .with_state(app_state.clone())
}

#[derive(Debug, Deserialize)]
pub struct Request {
    pub badge_family: BadgeFamily,
    pub badge: Option<String>,
}

#[derive(Debug, Serialize)]
pub enum Response {
    Family(Vec<String>),
    BadgeValue(EnumValue),
}

pub fn resolve<T>(badge: Option<String>) -> Result<Response, io::Error>
where
    T: FromStr + BadgeValue + IntoEnumIterator + Display,
    T::Err: Error + Sync + Send + 'static,
{
    Ok(match badge {
        Some(badge) => Response::BadgeValue(
            T::from_str(&badge)
                .map(|f| f.to_value().clone())
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?,
        ),
        None => Response::Family(T::iter().map(|f| f.to_string()).collect()),
    })
}

pub async fn root(Query(request): Query<Request>) -> Result<Json<Response>, AppError> {
    Ok(Json(match request.badge_family {
        BadgeFamily::ValutBadge => resolve::<ValutBadge>(request.badge)?,
        BadgeFamily::TradeBadge => resolve::<TradeBadge>(request.badge)?,
        BadgeFamily::TransferBadge => resolve::<TransferBadge>(request.badge)?,
        BadgeFamily::MakerBadge => resolve::<MakerBadge>(request.badge)?,
        BadgeFamily::TakerBadge => resolve::<TakerBadge>(request.badge)?,
    }))
}
