use fraction::num_traits::{CheckedAdd, CheckedSub};
use sqlx::{Postgres, Transaction};
use value::Value;

use super::models::transfer::{
    RevertTransferError, RevertTransferRequest, RevertTransferResponse, TransferError,
    TransferRequest, TransferResponse,
};
use crate::database::{
    managers::{transfers::TransfersManager, AssetsManager, ValutsManager},
    projections::transfer::Transfer,
};

pub async fn transfer<'t, 'p>(
    request: TransferRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
) -> Result<TransferResponse, TransferError> {
    if request.maker_id == request.taker_id {
        return Err(TransferError::SameUser);
    }

    let asset = AssetsManager::get_by_id(transaction, request.asset_id)
        .await?
        .ok_or(TransferError::AssetNotFound)?;

    let mut maker_asset_valut =
        ValutsManager::get_or_create(transaction, request.maker_id, asset.id).await?;

    let mut taker_asset_valut =
        ValutsManager::get_or_create(transaction, request.taker_id, asset.id).await?;

    let amount = Value::Finite(request.amount.to_owned());

    if maker_asset_valut.balance < amount {
        return Err(TransferError::InsufficientBalance);
    }

    // transfer
    maker_asset_valut.balance = maker_asset_valut
        .balance
        .checked_sub(&amount)
        .ok_or(TransferError::CheckedSubFailed)?;
    taker_asset_valut.balance = taker_asset_valut
        .balance
        .checked_add(&amount)
        .ok_or(TransferError::CheckedAddFailed)?;

    let transfer = Transfer {
        maker_id: request.maker_id,
        taker_id: request.taker_id,
        asset_id: asset.id,
        amount: request.amount,
    };

    let id = TransfersManager::insert(transaction, transfer).await?;
    ValutsManager::update(transaction, maker_asset_valut).await?;
    ValutsManager::update(transaction, taker_asset_valut).await?;

    Ok(TransferResponse { id })
}

pub async fn revert_transfer<'t, 'p>(
    request: RevertTransferRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
) -> Result<RevertTransferResponse, RevertTransferError> {
    let trans = TransfersManager::get_by_id(transaction, request.id).await?;
    let request = TransferRequest {
        maker_id: trans.taker_id,
        taker_id: trans.maker_id,
        asset_id: trans.asset_id,
        amount: trans.amount,
    };
    let response = transfer(request, transaction).await?;
    Ok(RevertTransferResponse { id: response.id })
}
