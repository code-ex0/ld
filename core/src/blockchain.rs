use crate::{Block, Payload};

#[derive(Debug, Clone, PartialEq)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub temp_payload: Vec<Payload>,
}

pub struct BlockchainIterator {
    pub blockchain: Blockchain,
    pub index: usize,
}

impl Iterator for BlockchainIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.blockchain.blocks.len() {
            let block = self.blockchain.blocks[self.index].clone();
            self.index += 1;
            Some(block)
        } else {
            None
        }
    }
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

impl IntoIterator for Blockchain {
    type Item = Block;
    type IntoIter = BlockchainIterator;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator {
            blockchain: self,
            index: 0,
        }
    }
}