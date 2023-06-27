#![feature(async_fn_in_trait)]

pub mod confirmation;
pub mod submission;
pub mod contracts;
pub mod database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let BLOCK_CONFIRMATIONS: usize = std::env::var("WORKER_FRACTION_ACCURACY")?.parse()?;

    Ok(())
}
