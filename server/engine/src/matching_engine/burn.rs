use chrono::{Duration, Utc};
use fraction::{
    num_traits::{CheckedSub, Zero},
    Fraction,
};
use sqlx::{Postgres, Transaction};

use super::models::burn::{BurnError, BurnRequest, BurnResponse};
use crate::database::{
    managers::{burns::BurnsManager, AssetsManager, ValutsManager},
    projections::burn::Burn,
};

pub async fn burn<'t, 'p>(
    request: BurnRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
    timeout: Duration,
) -> Result<BurnResponse, BurnError> {
    if request.amount <= Fraction::zero() {
        return Err(BurnError::InvalidAmount);
    }

    if let Some(last_burn) = BurnsManager::get_last_for_user(transaction, request.user_id).await? {
        if Utc::now() - last_burn.last_modification_at < timeout {
            return Err(BurnError::Timeout);
        }
    }

    let asset = AssetsManager::get_by_id(transaction, request.asset_id)
        .await?
        .ok_or(BurnError::AssetNotFound)?;

    let mut asset_valut =
        ValutsManager::get_or_create(transaction, request.user_id, asset.id).await?;

    if asset_valut.balance < request.amount {
        return Err(BurnError::InsufficientBalance);
    }

    // burn
    asset_valut.balance = asset_valut
        .balance
        .checked_sub(&request.amount)
        .ok_or(BurnError::CheckedSubFailed)?;

    BurnsManager::insert(
        transaction,
        Burn {
            user_id: request.user_id,
            asset_id: request.asset_id,
            amount: request.amount,
        },
    )
    .await?;
    ValutsManager::update(transaction, asset_valut).await?;

    Ok(BurnResponse {})
}
