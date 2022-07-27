use crate::{Block, BlockchainError, BlockError};
use crypto::digest::Digest;
use crypto::sha3::Sha3;

pub trait ProofOfWork {
    fn mine(&mut self, difficulty: u32);
    fn try_mine(&self, difficulty: u32) -> Result<Block, BlockError>;
    fn hash(&self) -> String;
    fn validate(&self) -> bool;
}

impl ProofOfWork for Block {
    fn mine(&mut self, difficulty: u32) {
        match self.try_mine(difficulty) {
            Ok(block) => {
                self.hash = block.hash;
                self.nonce = block.nonce;
            }
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }

    fn try_mine(&self, difficulty: u32) -> Result<Block, BlockError> {
        let mut block = self.clone();
        block.hash = block.hash();
        let difficulty_str = format!("{:0>width$}", "", width = difficulty as usize);
        while !block.hash.starts_with(&difficulty_str) {
            block.nonce += 1;
            block.hash = block.hash();
            if block.nonce > 1_000_000_000_000_000_000 {
                return Err(BlockError::InvalidHash);
            }
        }
        Ok(block)
    }

    fn hash(&self) -> String {
        let mut sha3 = Sha3::sha3_256();
        sha3.input_str(&self.get_previous_hash());
        //sha3.input_str(&self.get_payloads().to_string());
        sha3.input_str(&self.get_timestamp().to_string());
        sha3.input_str(&self.get_nonce().to_string());
        sha3.result_str()
    }

    fn validate(&self) -> bool {
        self.hash == self.hash()
    }
}