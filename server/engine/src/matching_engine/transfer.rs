use fraction::num_traits::{CheckedAdd, CheckedSub};
use sqlx::{Postgres, Transaction};

use super::models::transfer::{TransferError, TransferRequest, TransferResponse};
use crate::database::{
    managers::{transfers::TransfersManager, AssetsManager, ValutsManager},
    projections::transfer::Transfer,
};

pub async fn transfer<'t, 'p>(
    request: TransferRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
) -> Result<TransferResponse, TransferError> {
    if request.maker_id == request.taker_id {
        return Ok(TransferResponse {});
    }

    let asset = AssetsManager::get_by_id(transaction, request.asset_id)
        .await?
        .ok_or(TransferError::AssetNotFound)?;

    let mut maker_asset_valut =
        ValutsManager::get_or_create(transaction, request.maker_id, asset.id).await?;

    let mut taker_asset_valut =
        ValutsManager::get_or_create(transaction, request.taker_id, asset.id).await?;

    if maker_asset_valut.balance < request.amount {
        return Err(TransferError::InsufficientBalance);
    }

    // transfer
    maker_asset_valut.balance = maker_asset_valut
        .balance
        .checked_sub(&request.amount)
        .ok_or(TransferError::CheckedSubFailed)?;
    taker_asset_valut.balance = taker_asset_valut
        .balance
        .checked_add(&request.amount)
        .ok_or(TransferError::CheckedAddFailed)?;

    let transfer = Transfer {
        maker_id: request.maker_id,
        taker_id: request.taker_id,
        asset_id: asset.id,
        amount: request.amount,
    };

    TransfersManager::insert(transaction, transfer).await?;
    ValutsManager::update(transaction, maker_asset_valut).await?;
    ValutsManager::update(transaction, taker_asset_valut).await?;

    Ok(TransferResponse {})
}
