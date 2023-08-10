use chrono::{DateTime, Utc};
use linked_hash_map::LinkedHashMap;
use std::hash::Hash;
use uuid::Uuid;

use crate::database::projections::{
    withdraw::{Withdraw, WithdrawInsert},
    Expirable,
};

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct WithdrawQueueValue {
    pub withdraw: Withdraw,
    pub transfer: Uuid,
}

pub struct WithdrawQueue {
    entries: LinkedHashMap<WithdrawInsert, WithdrawQueueValue>,
}

impl WithdrawQueue {
    pub fn new() -> Self {
        Self {
            entries: LinkedHashMap::new(),
        }
    }

    pub fn insert(&mut self, key: WithdrawInsert, value: WithdrawQueueValue) -> bool {
        self.entries.insert(key, value).is_some()
    }

    pub fn remove(&mut self, key: &WithdrawInsert) -> bool {
        self.entries.remove(key).is_some()
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn eval(&mut self, time: &DateTime<Utc>) -> Vec<WithdrawQueueValue> {
        let mut expired = Vec::new();
        while let Some((_, value)) = self.entries.front() {
            if value.withdraw.is_expired(time) {
                if let Some((_, value)) = self.entries.pop_front() {
                    expired.push(value);
                }
            } else {
                break;
            }
        }
        expired
    }
}

impl Default for WithdrawQueue {
    fn default() -> Self {
        Self::new()
    }
}
