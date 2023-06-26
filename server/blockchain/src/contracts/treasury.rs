use std::{io, pin::Pin, sync::Arc};

use ethers::{
    contract::{EthLogDecode, Event},
    core::types,
    prelude::*,
    providers::{Http, Provider, Ws},
};

use super::abigen::treasury::Treasury;

pub struct TreasuryManager {
    http_provider: Provider<Http>,
    ws_provider: Provider<Ws>,
    contract: Treasury<Provider<Ws>>,
}

impl TreasuryManager {
    pub fn new(
        http_provider: Provider<Http>,
        ws_provider: Provider<Ws>,
        contract_address: types::Address,
    ) -> Self {
        Self {
            http_provider,
            ws_provider: ws_provider.clone(),
            contract: Treasury::new(contract_address, Arc::new(ws_provider)),
        }
    }
}
