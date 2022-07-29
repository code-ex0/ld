use serde::{Deserialize, Serialize};
use core::{Transaction, TransactionData};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransactionPost {
    pub sender: String,
    pub recipient: String,
    pub amount: u128,
    pub signature: String,
}

impl TransactionPost {
    pub fn into_transaction(&self) -> Transaction {
        Transaction {
            version: 1,
            transaction_data: TransactionData::Transfer { to: self.recipient.clone(), amount: self.amount },
            timestamp: 0,
            public_key: self.sender.clone().into_bytes(),
            signature: self.signature.clone().into_bytes(),
        }
    }
}