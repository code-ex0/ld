use crate::{Block, Payload};

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain{
    pub blocks: Vec<Block>,
    pub temp_payload: Vec<Payload>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: Vec::new(),
            temp_payload: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn add_payload(&mut self, payload: Payload) {
        self.temp_payload.push(payload);
    }
}
