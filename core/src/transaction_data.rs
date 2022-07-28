use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionData {
    Transfer {to: String, amount: u128},
    Create {receiver: String,amount: u128},
}