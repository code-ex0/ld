use std::sync::{Arc, Mutex};
use core::{Blockchain, Block};
use core::Transactions;
use core::ProofOfWork;

pub type SyncBlockchain = Arc<Mutex<Blockchain>>;

#[derive(Clone)]
pub struct BlockchainPool {
    pub blockchain: SyncBlockchain,
}

impl BlockchainPool {
    pub fn new() -> BlockchainPool {
        BlockchainPool {
            blockchain: Arc::new(Mutex::new(Blockchain::new())),
        }
    }

    pub fn add_block(&self, block: Block) {
        self.blockchain.lock().unwrap().add_block(block);
    }

    pub fn new_block(&self, transactions: Transactions) -> Block {
        let blockchain = self.blockchain.lock().unwrap();
        return if let Ok(last_block) = blockchain.last_block() {
            Block::new(last_block.id + 1, 0, transactions, last_block.hash.clone(), 0)
        } else {
            Block::new(0, 0, transactions, "genesis".to_string(), 0)
        }
    }

    // check if last block is valid
    pub fn is_valid(&self) -> bool {
        let blockchain = self.blockchain.lock().unwrap();

        if let Ok(last_block) = blockchain.last_block() {
            return last_block.validate();
        }

        if blockchain.blocks.is_empty() {
            return true;
        }

        return false;
    }
}
