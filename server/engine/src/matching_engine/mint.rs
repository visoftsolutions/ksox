use chrono::{Duration, Utc};
use fraction::{
    num_traits::{CheckedAdd, Zero},
    Fraction,
};
use sqlx::{Postgres, Transaction};

use super::models::mint::{MintError, MintRequest, MintResponse};
use crate::database::{
    managers::{mints::MintsManager, AssetsManager, ValutsManager},
    projections::mint::Mint,
};

pub async fn mint<'t, 'p>(
    request: MintRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
    timeout: Duration,
) -> Result<MintResponse, MintError> {
    if request.amount <= Fraction::zero() {
        return Err(MintError::InvalidAmount);
    }

    if let Some(last_mint) = MintsManager::get_last_for_user(transaction, request.user_id).await? {
        if Utc::now() - last_mint.last_modification_at < timeout {
            return Err(MintError::Timeout);
        }
    }

    let asset = AssetsManager::get_by_id(transaction, request.asset_id)
        .await?
        .ok_or(MintError::AssetNotFound)?;

    let mut asset_valut =
        ValutsManager::get_or_create(transaction, request.user_id, asset.id).await?;

    // mint
    asset_valut.balance = asset_valut
        .balance
        .checked_add(&request.amount)
        .ok_or(MintError::CheckedSubFailed)?;

    MintsManager::insert(
        transaction,
        Mint {
            user_id: request.user_id,
            asset_id: request.asset_id,
            amount: request.amount,
        },
    )
    .await?;
    ValutsManager::update(transaction, asset_valut).await?;

    Ok(MintResponse {})
}
