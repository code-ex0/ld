use crate::payload::Payload;

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub id: u64,
    pub timestamp: u64,
    pub payloads: Vec<Payload>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

impl Block {
    pub fn new(id: u64, timestamp: u64, payloads: Vec<Payload>, previous_hash: String, nonce: u64) -> Block {
        Block {
            id,
            timestamp,
            payloads,
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

    pub fn get_payloads(&self) -> &Vec<Payload> {
        &self.payloads
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