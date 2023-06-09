use sqlx::{Postgres, Transaction};

use super::models::cancel::{CancelRequest, CancelRequestError, CancelResponse};
use crate::database::managers::{OrdersManager, ValutsManager};

pub async fn cancel<'t, 'p>(
    request: CancelRequest,
    transaction: &'t mut Transaction<'p, Postgres>,
) -> Result<CancelResponse, CancelRequestError> {
    let mut order = OrdersManager::get_by_id(transaction, request.order_id)
        .await?
        .ok_or(CancelRequestError::OrderNotFound)?;

    if !order.is_active {
        return Err(CancelRequestError::OrderNotActive);
    }

    let mut valut =
        ValutsManager::get_or_create(transaction, order.maker_id, order.quote_asset_id).await?;

    valut.balance += order.quote_asset_volume_left.to_owned();
    order.is_active = false;

    ValutsManager::update(transaction, valut).await?;
    OrdersManager::update_status(transaction, order.into()).await?;

    Ok(CancelResponse {})
}
