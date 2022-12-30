use crate::{Transactions};
use serde::{Deserialize, Serialize};

pub type Blocks = Vec<Block>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub transactions: Transactions,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, timestamp: u64, transactions: Transactions, previous_hash: String, nonce: u64) -> Block {
        Block {
            id,
            timestamp,
            transactions,
            previous_hash,
            hash: String::new(),
            nonce,
        }
    }

    ///
    /// getter
    ///
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_payloads(&self) -> &Transactions {
        &self.transactions
    }

    pub fn get_previous_hash(&self) -> &String {
        &self.previous_hash
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce
    }
}