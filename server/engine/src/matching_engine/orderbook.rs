use std::pin::Pin;

use database::{
    managers::spot::orders::OrdersManager,
    projections::spot::order::Order,
    sqlx::{
        types::{BigDecimal, Uuid},
        Pool, Postgres, Result,
    },
    traits::{manager::Manager, ordered_manager::OrderedManager},
};
use futures::Stream;

pub struct OrderBook {
    quote_asset_id: Uuid,
    base_asset_id: Uuid,
    manager: OrdersManager,
}

impl OrderBook {
    pub fn new(database: Pool<Postgres>, quote_asset_id: Uuid, base_asset_id: Uuid) -> OrderBook {
        OrderBook {
            manager: OrdersManager::new(database),
            quote_asset_id,
            base_asset_id,
        }
    }

    pub async fn place_order(&self, order: Order) -> Result<()> {
        todo!()
    }

    pub async fn remove_order(&self, order: Order) -> Result<()> {
        todo!()
    }

    pub fn get_ordered_asc_less(
        &self,
        less_then: BigDecimal,
    ) -> Pin<Box<dyn Stream<Item = Result<Order>> + Send + '_>> {
        self.manager.get_ordered_asc_less(less_then)
    }
}
