use fraction::{
    num_traits::{CheckedAdd, CheckedSub, One, Zero},
    Fraction,
};
use num_traits::{CheckedDiv, CheckedMul};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
use value::Value;

use super::models::transfer::{
    RevertTransferError, RevertTransferResponse, TransferError, TransferRequest, TransferResponse,
};
use crate::database::{
    managers::{transfers::TransfersManager, AssetsManager, ValutsManager},
    projections::transfer::Transfer,
};

pub async fn transfer<'t>(
    request: &TransferRequest,
    fee_harvester_user_id: &Uuid,
    transaction: &'t mut Transaction<'_, Postgres>,
) -> Result<TransferResponse, TransferError> {
    let asset = AssetsManager::get_by_id(transaction, &request.asset_id)
        .await?
        .ok_or(TransferError::AssetNotFound)?;

    let fee = request
        .amount
        .checked_mul(&request.fee_fraction)
        .ok_or(TransferError::CheckedMulFailed)?
        .checked_floor_with_accuracy(
            &Fraction::one()
                .checked_div(&asset.decimals)
                .ok_or(TransferError::CheckedDivFailed)?,
        )
        .ok_or(TransferError::CheckedFloorFailed)?;

    let mut from_valut = ValutsManager::get_by_id(transaction, &request.from_valut_id).await?;
    from_valut.balance = from_valut
        .balance
        .checked_sub(&Value::Finite(request.amount.to_owned()))
        .ok_or(TransferError::CheckedSubFailed)?;
    if from_valut.balance <= Value::Finite(Fraction::zero()) {
        return Err(TransferError::InsufficientBalance);
    }
    ValutsManager::update(transaction, from_valut).await?;

    let mut to_valut = ValutsManager::get_by_id(transaction, &request.to_valut_id).await?;
    to_valut.balance = to_valut
        .balance
        .checked_add(&Value::Finite(request.amount.to_owned()))
        .ok_or(TransferError::CheckedAddFailed)?;
    ValutsManager::update(transaction, to_valut).await?;

    let mut fee_provider_valut =
        ValutsManager::get_by_id(transaction, &request.from_valut_id).await?;
    fee_provider_valut.balance = fee_provider_valut
        .balance
        .checked_sub(&Value::Finite(fee.to_owned()))
        .ok_or(TransferError::CheckedSubFailed)?;
    if fee_provider_valut.balance <= Value::Finite(Fraction::zero()) {
        return Err(TransferError::InsufficientBalanceFee);
    }
    ValutsManager::update(transaction, fee_provider_valut).await?;

    let mut fee_harvester_valut =
        ValutsManager::get_or_create(transaction, fee_harvester_user_id, &asset.id).await?;
    fee_harvester_valut.balance = fee_harvester_valut
        .balance
        .checked_add(&Value::Finite(fee.to_owned()))
        .ok_or(TransferError::CheckedAddFailed)?;
    ValutsManager::update(transaction, fee_harvester_valut).await?;

    let transfer = Transfer {
        from_valut_id: request.from_valut_id.to_owned(),
        to_valut_id: request.to_valut_id.to_owned(),
        fee_harvester_user_id: fee_harvester_user_id.to_owned(),
        asset_id: asset.id.to_owned(),
        amount: request.amount.to_owned(),
        fee,
    };

    let transfer_id = TransfersManager::insert(transaction, transfer).await?;

    Ok(TransferResponse { transfer_id })
}

pub async fn revert_transfer<'t>(
    transfer_id: Uuid,
    transaction: &'t mut Transaction<'_, Postgres>,
) -> Result<RevertTransferResponse, RevertTransferError> {
    let revert_transfer = TransfersManager::get_by_id(transaction, transfer_id).await?;
    let amount_transfer = transfer(
        &TransferRequest {
            from_valut_id: revert_transfer.to_valut_id,
            to_valut_id: revert_transfer.from_valut_id,
            asset_id: revert_transfer.asset_id,
            amount: revert_transfer.amount,
            fee_fraction: Fraction::zero(),
        },
        &Uuid::default(),
        transaction,
    )
    .await?;
    let fee_harvester_valut = ValutsManager::get_or_create(
        transaction,
        &revert_transfer.fee_harvester_user_id,
        &revert_transfer.asset_id,
    )
    .await?;
    let fee_transfer = transfer(
        &TransferRequest {
            from_valut_id: fee_harvester_valut.id,
            to_valut_id: revert_transfer.from_valut_id,
            asset_id: revert_transfer.asset_id,
            amount: revert_transfer.fee,
            fee_fraction: Fraction::zero(),
        },
        &Uuid::default(),
        transaction,
    )
    .await?;

    Ok(RevertTransferResponse {
        amount_transfer_id: amount_transfer.transfer_id,
        fee_transfer_id: fee_transfer.transfer_id,
    })
}
