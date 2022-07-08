use crate::{Block, Blocks, Payload};

#[derive(Debug, Clone, PartialEq)]
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

    pub fn add_payload(&mut self, payload: Payload) {
        self.temp_payload.push(payload);
    }
}
