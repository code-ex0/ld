pub mod block;
pub use block::Block;

pub mod payload;
pub use payload::Payload;

pub mod blockchain;
pub use blockchain::Blockchain;


pub struct BlockchainIterator<'a> {
    pub blockchain: &'a Blockchain,
    pub index: usize,
}

impl<'a> IntoIterator for &'a Blockchain {
    type Item = Block;
    type IntoIter = BlockchainIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BlockchainIterator {
            blockchain: self,
            index: 0,
        }
    }
}


impl<'a> Iterator for BlockchainIterator<'a>  {
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
