use fraction::{
    num_traits::{CheckedAdd, Zero},
    Fraction,
};
use sqlx::{Postgres, Transaction};
use value::Value;

use super::models::mint::{MintError, MintRequest, MintResponse};
use crate::database::managers::{AssetsManager, ValutsManager};

pub async fn mint<'t, 'p>(
    request: MintRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
) -> Result<MintResponse, MintError> {
    if request.amount <= Fraction::zero() {
        return Err(MintError::InvalidAmount);
    }

    let asset = AssetsManager::get_by_id(transaction, request.asset_id)
        .await?
        .ok_or(MintError::AssetNotFound)?;

    let mut asset_valut =
        ValutsManager::get_or_create(transaction, request.user_id, asset.id).await?;

    let amount = Value::Finite(request.amount);

    // mint
    asset_valut.balance = asset_valut
        .balance
        .checked_add(&amount)
        .ok_or(MintError::CheckedSubFailed)?;

    ValutsManager::update(transaction, asset_valut).await?;

    Ok(MintResponse {})
}
