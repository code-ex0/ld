mod block;
pub use block::Block;
pub use block::Blocks;

mod transaction;
pub use transaction::Transaction;
pub use transaction::Transactions;

mod blockchain;
pub use blockchain::Blockchain;

mod blockchain_error;
pub use blockchain_error::BlockchainError;

mod block_error;
pub use block_error::BlockError;

mod proof_of_work;
pub use proof_of_work::ProofOfWork;

mod transaction_data;
pub use transaction_data::TransactionData;

mod transaction_error;
pub use transaction_error::TransactionError;


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
