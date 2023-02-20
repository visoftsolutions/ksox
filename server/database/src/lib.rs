#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
pub mod managers;
pub mod projections;
pub mod traits;
pub mod types;

pub use sqlx;

#[cfg(test)]
pub mod tests;
