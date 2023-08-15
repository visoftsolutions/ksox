use fraction::{
    num_traits::{CheckedAdd, CheckedSub},
    Fraction,
};
use num_traits::Zero;
use num_traits::{CheckedMul, Inv};
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
    fee_valut_id: &Uuid,
    transaction: &'t mut Transaction<'_, Postgres>,
) -> Result<TransferResponse, TransferError> {
    if request.from_valut_id == request.to_valut_id
        || request.to_valut_id == *fee_valut_id
        || *fee_valut_id == request.from_valut_id
    {
        return Err(TransferError::SameValut);
    }

    let asset = AssetsManager::get_by_id(transaction, &request.asset_id)
        .await?
        .ok_or(TransferError::AssetNotFound)?;

    tracing::info!("{}, {}, {}", request.from_valut_id, request.to_valut_id, fee_valut_id);
    let mut from_valut = ValutsManager::get_by_id(transaction, &request.from_valut_id).await?;
    let mut to_valut = ValutsManager::get_by_id(transaction, &request.to_valut_id).await?;
    let mut fee_valut = ValutsManager::get_by_id(transaction, &fee_valut_id).await?;
    tracing::info!("works");

    let reduced_amount = request
        .amount
        .checked_sub(
            &request
                .amount
                .checked_mul(&request.fee_fraction)
                .ok_or(TransferError::CheckedMulFailed)?,
        )
        .ok_or(TransferError::CheckedSubFailed)?
        .checked_ceil_with_accuracy(&asset.decimals.inv())
        .ok_or(TransferError::CheckedCeilFailed)?;

    let fee = request
        .amount
        .checked_sub(&reduced_amount)
        .ok_or(TransferError::CheckedSubFailed)?;

    let transfer = Transfer {
        from_valut_id: request.from_valut_id.to_owned(),
        to_valut_id: request.to_valut_id.to_owned(),
        fee_valut_id: fee_valut_id.to_owned(),
        asset_id: request.asset_id.to_owned(),
        amount: request.amount.to_owned(),
        fee: fee.to_owned(),
    };

    // transfer
    from_valut.balance = from_valut
        .balance
        .checked_sub(&Value::Finite(request.amount.to_owned()))
        .ok_or(TransferError::CheckedSubFailed)?;

    to_valut.balance = to_valut
        .balance
        .checked_add(&Value::Finite(reduced_amount))
        .ok_or(TransferError::CheckedAddFailed)?;

    fee_valut.balance = fee_valut
        .balance
        .checked_add(&Value::Finite(fee))
        .ok_or(TransferError::CheckedAddFailed)?;

    tracing::info!("works1");
    let transfer_id = TransfersManager::insert(transaction, transfer).await?;
    tracing::info!("works2");

    ValutsManager::update(transaction, from_valut).await?;
    ValutsManager::update(transaction, to_valut).await?;
    ValutsManager::update(transaction, fee_valut).await?;
    tracing::info!("works3");

    Ok(TransferResponse { transfer_id })
}

pub async fn revert_transfer<'t>(
    transfer_id: Uuid,
    t: &'t mut Transaction<'_, Postgres>,
) -> Result<RevertTransferResponse, RevertTransferError> {
    let revert_transfer = TransfersManager::get_by_id(t, transfer_id).await?;
    let amount_transfer = transfer(
        &TransferRequest {
            from_valut_id: revert_transfer.to_valut_id,
            to_valut_id: revert_transfer.from_valut_id,
            asset_id: revert_transfer.asset_id,
            amount: revert_transfer.amount,
            fee_fraction: Fraction::zero(),
        },
        &revert_transfer.fee_valut_id,
        t,
    )
    .await?;

    let fee_transfer = transfer(
        &TransferRequest {
            from_valut_id: revert_transfer.fee_valut_id,
            to_valut_id: revert_transfer.from_valut_id,
            asset_id: revert_transfer.asset_id,
            amount: revert_transfer.fee,
            fee_fraction: Fraction::zero(),
        },
        &Uuid::default(),
        t,
    )
    .await?;

    Ok(RevertTransferResponse {
        amount_transfer_id: amount_transfer.transfer_id,
        fee_transfer_id: fee_transfer.transfer_id,
    })
}
