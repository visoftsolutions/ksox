use chrono::{DateTime, Utc};

pub mod asset;
pub mod deposit;
pub mod user;
pub mod valut;
pub mod withdraw;

pub trait Confirmable {
    fn set(&mut self, confirmations: u64);
    fn is_confirmed(&self) -> bool;
}

pub trait Expirable {
    fn is_expired(&self, time: &DateTime<Utc>) -> bool;
}
