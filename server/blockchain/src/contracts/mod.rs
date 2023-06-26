pub mod abigen;
pub mod treasury;
use std::pin::Pin;

use async_stream::try_stream;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
};
use tokio_stream::{Stream, StreamExt};

pub fn block_confirmations<'provider>(
    ws_provider: &'provider Provider<Ws>,
    block_hash: H256,
) -> Pin<Box<dyn Stream<Item = Result<usize, ProviderError>> + Send + 'provider>> {
    Box::pin(try_stream! {
        while let Some(current_block) =  ws_provider.subscribe_blocks().await?.next().await {
            let observed_block = ws_provider.get_block(block_hash).await?
                .ok_or(ProviderError::CustomError("block not found".to_string()))?;

            if let (Some(current_block_number), Some(observed_block_number)) = (current_block.number, observed_block.number) {
                yield (current_block_number - observed_block_number).as_usize();
            }
        }
    })
}

pub fn tx_confirmations<'provider>(
    ws_provider: &'provider Provider<Ws>,
    tx_hash: H256,
) -> Pin<Box<dyn Stream<Item = Result<usize, ProviderError>> + Send + 'provider>> {
    Box::pin(try_stream! {
        while let Some(current_block) =  ws_provider.subscribe_blocks().await?.next().await {
            let transaction = ws_provider.get_transaction(tx_hash).await?
                .ok_or(ProviderError::CustomError("transaction not found".to_string()))?;

            let observed_block = ws_provider.get_block(
                transaction.block_hash.ok_or(ProviderError::CustomError("transaction not in block".to_string()))?
            ).await?.ok_or(ProviderError::CustomError("block not found".to_string()))?;

            if let (Some(current_block_number), Some(observed_block_number)) = (current_block.number, observed_block.number) {
                yield (current_block_number - observed_block_number).as_usize();
            }
        }
    })
}
