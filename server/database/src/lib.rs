#![allow(dead_code)]
#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
#![feature(result_option_inspect)]
pub mod managers;
pub mod projections;
pub mod traits;
pub mod types;
pub mod utils;

pub use sqlx;
