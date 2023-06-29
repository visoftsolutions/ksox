use std::{
    io,
    ops::{Add, AddAssign},
    pin::Pin,
};

use async_stream::try_stream;
use fraction::Fraction;
use num_traits::Inv;
use serde::{Deserialize, Serialize};
use tokio_stream::{Stream, StreamExt};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct PriceLevel {
    pub price: Fraction,
    pub volume: Fraction,
}

impl PriceLevel {
    pub fn checked_round_with_accuracy(self, accuracy: &Fraction) -> Option<Self> {
        Some(Self {
            price: self.price.checked_round_with_accuracy(accuracy)?,
            volume: self.volume.checked_round_with_accuracy(accuracy)?,
        })
    }
}

impl Inv for PriceLevel {
    type Output = Self;
    fn inv(self) -> Self::Output {
        Self {
            price: self.price.clone().inv(),
            volume: self.volume / self.price,
        }
    }
}

impl Add for PriceLevel {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            price: self.price,
            volume: self.volume + rhs.volume,
        }
    }
}

impl AddAssign for PriceLevel {
    fn add_assign(&mut self, rhs: Self) {
        self.volume += rhs.volume;
    }
}

impl PartialOrd for PriceLevel {
    fn ge(&self, other: &Self) -> bool {
        self.price.ge(&other.price)
    }
    fn gt(&self, other: &Self) -> bool {
        self.price.gt(&other.price)
    }
    fn le(&self, other: &Self) -> bool {
        self.price.le(&other.price)
    }
    fn lt(&self, other: &Self) -> bool {
        self.price.lt(&other.price)
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl PriceLevel {
    pub fn aggregate_with_accuracy(
        mut ordered_price_level_stream: Pin<Box<dyn Stream<Item = io::Result<Self>> + Send + '_>>,
        accuracy: Fraction,
    ) -> Pin<Box<dyn Stream<Item = io::Result<Self>> + Send + '_>> {
        Box::pin(try_stream! {
            let mut agg:Option<PriceLevel> = None;

            while let Some(price_level) = ordered_price_level_stream.next().await {
                let price_level = price_level?.checked_round_with_accuracy(&accuracy).ok_or(io::Error::new(io::ErrorKind::InvalidData, "checked_round_with_accuracy failed"))?;
                match agg {
                    Some(agg_price_level) => {
                        if agg_price_level > price_level {
                            yield agg_price_level.clone();
                            agg = Some(price_level);
                        } else {
                            agg = Some(agg_price_level + price_level);
                        }
                    },
                    None => {
                        agg = Some(price_level);
                    }
                }
            }

            if let Some(agg_price_level) = agg {
                yield agg_price_level;
            }
        })
    }
}
