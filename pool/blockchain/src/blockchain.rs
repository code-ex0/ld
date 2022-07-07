use std::sync::{Arc, Mutex};
use core::{blockchain::Blockchain, block::Block};

pub type SyncBlockchain = Arc<Mutex<Blockchain>>;

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
        let last_block = blockchain.blocks.last().unwrap();
        Block::new(last_block.id + 1, 0, vec![], last_block.hash.clone(), 0)
    }
}
