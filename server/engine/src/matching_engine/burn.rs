use fraction::{
    num_traits::{CheckedSub, Zero},
    Fraction,
};
use sqlx::{Postgres, Transaction};

use super::models::burn::{BurnError, BurnRequest, BurnResponse};
use crate::database::managers::{AssetsManager, ValutsManager};

pub async fn burn<'t, 'p>(
    request: BurnRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
) -> Result<BurnResponse, BurnError> {
    if request.amount <= Fraction::zero() {
        return Err(BurnError::InvalidAmount);
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

    ValutsManager::update(transaction, asset_valut).await?;

    Ok(BurnResponse {})
}
