use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use super::valut::Valut;
use crate::types::{fraction::FractionError, Fraction, Volume};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub user_id: Uuid,
    pub is_active: bool,
    pub quote_asset_id: Uuid,
    pub base_asset_id: Uuid,
    pub quote_asset_volume: Volume,
    pub base_asset_volume: Volume,
    pub quote_asset_volume_left: Volume,
    pub maker_fee_num: Volume,
    pub maker_fee_denum: Volume,
}

impl Order {
    pub fn base_asset_volume_left_floor(&self) -> Volume {
        self.base_asset_volume.to_owned() * self.quote_asset_volume_left.to_owned()
            / self.quote_asset_volume.to_owned()
    }

    pub fn base_asset_volume_left_ceil(&self) -> Volume {
        (self.base_asset_volume.to_owned() * self.quote_asset_volume_left.to_owned()
            + self.quote_asset_volume.to_owned()
            - Volume::from(1))
            / self.quote_asset_volume.to_owned()
    }

    pub fn fillable(&self) -> bool {
        self.quote_asset_volume > Volume::from(0)
            && self.base_asset_volume_left_floor() > Volume::from(0)
    }

    pub fn cancel(&mut self, valut: &mut Valut) {
        valut.balance += self.quote_asset_volume_left.to_owned();
        self.is_active = false;
    }

    pub fn maker_fee(&self) -> Result<Fraction, FractionError> {
        (
            self.maker_fee_num.clone().into(),
            self.maker_fee_denum.clone().into(),
        )
            .try_into()
    }
}
