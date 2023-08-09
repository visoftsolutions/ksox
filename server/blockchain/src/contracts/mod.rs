pub mod treasury;
use std::pin::Pin;

use async_stream::try_stream;
use ethers::{
    prelude::*,
    providers::{Provider, Ws},
};
use tokio_stream::{Stream, StreamExt};

use ethers::{
    contract::{Eip712, EthAbiType},
    core::types::{Address, U256},
};

#[derive(Eip712, EthAbiType, Clone)]
#[eip712(
    name = "Treasury",
    version = "1",
    chain_id = 11155111,
    verifying_contract = "0x09032B61218Be2906525696C97dC68f14BACdA1e"
)]
pub struct WithdrawPermit {
    pub owner: Address,
    pub spender: Address,
    pub token: Address,
    pub value: U256,
    pub nonce: U256,
    pub deadline: U256,
}

pub async fn transaction_block(
    ws_provider: &Provider<Ws>,
    tx_hash: TxHash,
) -> Result<Block<H256>, ProviderError> {
    let transaction_block_hash = ws_provider
        .get_transaction(tx_hash)
        .await?
        .and_then(|f| f.block_hash)
        .ok_or_else(|| ProviderError::CustomError("transaction not found".to_string()))?;
    ws_provider
        .get_block(transaction_block_hash)
        .await?
        .ok_or_else(|| ProviderError::CustomError("block not found".to_string()))
}

pub async fn current_block(ws_provider: &Provider<Ws>) -> Result<Block<H256>, ProviderError> {
    let block_number = ws_provider.get_block_number().await?;
    ws_provider
        .get_block(block_number)
        .await?
        .ok_or(ProviderError::CustomError("block not found".to_string()))
}

pub async fn block_distance(
    block_lhs: &Block<H256>,
    block_rhs: &Block<H256>,
) -> Result<usize, ProviderError> {
    if let (Some(block_lhs_number), Some(block_rhs_number)) = (block_lhs.number, block_rhs.number) {
        Ok((block_lhs_number.abs_diff(block_rhs_number))
            .try_into()
            .map_err(|e: &str| ProviderError::CustomError(e.to_string()))?)
    } else {
        Err(ProviderError::CustomError(
            "could not get block number".to_string(),
        ))
    }
}

pub fn block_distance_stream<'provider>(
    ws_provider: &'provider Provider<Ws>,
    block_rhs: Block<H256>,
) -> Pin<Box<dyn Stream<Item = Result<usize, ProviderError>> + Send + 'provider>> {
    Box::pin(try_stream! {
        while let Some(block_lhs) =  ws_provider.subscribe_blocks().await?.next().await {
            yield block_distance(&block_lhs, &block_rhs).await?;
        }
    })
}
