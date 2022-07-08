use std::sync::{Arc, Mutex};
use core::{Blockchain, Block};

pub type SyncBlockchain = Arc<Mutex<Blockchain>>;

#[derive(Debug)]
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

    pub fn new_block(&self) -> Block {
        let blockchain = self.blockchain.lock().unwrap();
        return if let Ok(last_block) = blockchain.last_block() {
            Block::new(last_block.id + 1, 0, vec![], last_block.hash.clone(), 0)
        } else {
            println!("Error: Blockchain is empty");
            Block::new(0, 0, vec![], "".to_string(), 0)
        }
    }
}
