use crate::{Block, Blocks, BlockchainError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Blockchain{
    pub blocks: Blocks,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn last_block(&self) -> Result<&Block, BlockchainError> {
        if self.blocks.is_empty() {
            Err(BlockchainError::EmptyBlockchain)?;
        }
        Ok(self.blocks.last().unwrap())
    }

    pub fn get_block(&self, id: u64) -> Result<Block, BlockchainError> {
        if self.blocks.is_empty() {
            Err(BlockchainError::EmptyBlockchain)?;
        }
        let block = self.blocks.iter().find(|block| block.get_id() == id);
        if block.is_none() {
            Err(BlockchainError::BlockNotFound)?;
        }
        Ok(block.unwrap().clone())
    }

}
