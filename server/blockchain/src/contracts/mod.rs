pub mod treasury;
use std::pin::Pin;

use async_stream::try_stream;
use ethers::{
    abi::{ParamType, Token},
    prelude::*,
    providers::{Provider, Ws},
    types::{
        transaction::eip712::{make_type_hash, EIP712Domain, Eip712, Eip712Error},
        U256,
    },
    utils::keccak256,
};
use evm::address::Address;
use tokio_stream::{Stream, StreamExt};

#[derive(Debug, Clone)]
pub struct Permit {
    pub chain_id: U256,
    pub name: String,
    pub verifying_contract: Address,
    pub owner: Address,
    pub spender: Address,
    pub token: Address,
    pub value: U256,
    pub nonce: U256,
    pub deadline: U256,
}
impl Eip712 for Permit {
    type Error = Eip712Error;

    fn domain(&self) -> Result<EIP712Domain, Self::Error> {
        Ok(EIP712Domain {
            name: Some(self.name.to_owned()),
            version: Some("1".to_string()),
            chain_id: Some(U256::from(31337)),
            verifying_contract: Some(self.verifying_contract.to_owned().into()),
            salt: None,
        })
    }

    fn type_hash() -> Result<[u8; 32], Self::Error> {
        let primary_type = "Permit".to_string();
        let fields = vec![
            ("owner".to_string(), ParamType::Address),
            ("spender".to_string(), ParamType::Address),
            ("token".to_string(), ParamType::Address),
            ("value".to_string(), ParamType::Uint(256)),
            ("nonce".to_string(), ParamType::Uint(256)),
            ("deadline".to_string(), ParamType::Uint(256)),
        ];
        Ok(make_type_hash(primary_type, fields.as_slice()))
    }

    fn struct_hash(&self) -> Result<[u8; 32], Self::Error> {
        let tokens = vec![
            Token::FixedBytes(Self::type_hash()?.to_vec()),
            Token::Address(self.owner.to_owned().into()),
            Token::Address(self.spender.to_owned().into()),
            Token::Address(self.token.to_owned().into()),
            Token::Uint(self.value),
            Token::Uint(self.nonce),
            Token::Uint(self.deadline),
        ];
        Ok(keccak256(abi::encode(tokens.as_slice())))
    }
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
