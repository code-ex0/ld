use crate::TransactionData;

pub type Transactions = Vec<Transaction>;

#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    pub version: u32,
    pub transaction_data: TransactionData,
    pub timestamp: u64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

impl Transaction {
    pub fn new(version: u32,transaction_data: TransactionData, timestamp: u64, public_key: Vec<u8>, signature: Vec<u8>) -> Transaction {
        Transaction {
            version,
            timestamp,
            public_key,
            signature,
            transaction_data,
        }
    }

    ///
    /// getter
    ///
    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_transaction_data(&self) -> &TransactionData {
        &self.transaction_data
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_public_key(&self) -> &Vec<u8> {
        &self.public_key
    }

    pub fn get_signature(&self) -> &Vec<u8> {
        &self.signature
    }
}