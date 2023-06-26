pub mod contracts;
pub mod database;
pub mod confirmation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let BLOCK_CONFIRMATIONS: usize = std::env::var("WORKER_FRACTION_ACCURACY")?.parse()?;

    Ok(())
}
