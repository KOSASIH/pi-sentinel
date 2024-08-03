use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct Genesis {
    pub timestamp: u64,
    pub block_number: u64,
    pub difficulty: u64,
    pub gas_limit: u64,
    pub gas_price: u64,
    pub alloc: HashMap<String, Account>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub balance: u64,
    pub nonce: u64,
}

impl Genesis {
    pub fn new() -> Self {
        Genesis {
            timestamp: 1643723400, // January 25, 2022, 12:00:00 PM UTC
            block_number: 0,
            difficulty: 1000,
            gas_limit: 100000,
            gas_price: 20,
            alloc: HashMap::new(),
        }
    }

    pub fn add_account(&mut self, address: &str, balance: u64) {
        self.alloc.insert(address.to_string(), Account { balance, nonce: 0 });
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
            }
