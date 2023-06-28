pub mod treasury;
use std::pin::Pin;

use async_stream::try_stream;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
};
use tokio_stream::{Stream, StreamExt};

pub fn block_confirmations_stream<'provider>(
    ws_provider: &'provider Provider<Ws>,
    block_hash: H256,
) -> Pin<Box<dyn Stream<Item = Result<usize, ProviderError>> + Send + 'provider>> {
    Box::pin(try_stream! {
        while let Some(current_block) =  ws_provider.subscribe_blocks().await?.next().await {
            let observed_block = ws_provider.get_block(block_hash).await?
                .ok_or(ProviderError::CustomError("block not found".to_string()))?;

            yield confirmations(&current_block,&observed_block).await?;
        }
    })
}

pub fn tx_confirmations_stream<'provider>(
    ws_provider: &'provider Provider<Ws>,
    tx_hash: H256,
) -> Pin<Box<dyn Stream<Item = Result<usize, ProviderError>> + Send + 'provider>> {
    Box::pin(try_stream! {
        while let Some(current_block) =  ws_provider.subscribe_blocks().await?.next().await {
            let observed_block = transaction_block(ws_provider, tx_hash).await?;
            yield confirmations(&current_block,&observed_block).await?;
        }
    })
}

pub async fn transaction_block<'provider>(
    ws_provider: &'provider Provider<Ws>,
    tx_hash: H256,
) -> Result<Block<H256>, ProviderError> {
    let transaction_block_hash = ws_provider
        .get_transaction(tx_hash)
        .await?
        .and_then(|f| f.block_hash)
        .ok_or_else(|| ProviderError::CustomError("transaction not found".to_string()))?;
    Ok(ws_provider
        .get_block(transaction_block_hash)
        .await?
        .ok_or_else(|| ProviderError::CustomError("block not found".to_string()))?)
}

pub async fn confirmations(
    block: &Block<H256>,
    tx_block: &Block<H256>,
) -> Result<usize, ProviderError> {
    if let (Some(block_number), Some(transaction_block_number)) = (block.number, tx_block.number) {
        Ok((block_number - transaction_block_number).as_usize())
    } else {
        Err(ProviderError::CustomError(
            "could not get block number".to_string(),
        ))
    }
}
