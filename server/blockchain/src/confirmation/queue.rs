use std::collections::HashSet;

use ethereum_types::H256;
use ethers::types::Block;

use crate::database::{managers::deposits::DepositsManager, projections::deposit::Deposit};

#[derive(Eq, Hash, PartialEq)]
pub struct ConfirmationQueueEntry {
    deposit: Deposit,
}

pub struct ConfirmationQueue {
    manager: DepositsManager,
    entry_set: HashSet<ConfirmationQueueEntry>
}

impl ConfirmationQueue {
    pub fn new(manager: DepositsManager) -> Self {
        Self { manager, entry_set: HashSet::new() }
    }

    pub fn insert(&mut self, entry: ConfirmationQueueEntry) -> bool {
        self.entry_set.insert(entry)
    }

    pub fn confirmation_step(&self, block: Block<H256>) -> () {
        
    }   
}