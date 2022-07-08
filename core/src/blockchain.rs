use crate::{Block, BlockchainError};

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain{
    pub blocks: Vec<Block>,
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

}
