pub use core::{Block, Payload, Blockchain};
pub use pool_blockchain::BlockchainPool;

fn main() {
    let mut blockchain = BlockchainPool::new();
    let genesis_block = Block::new(0, 0, vec![], "genesis".to_string(), 0);
    blockchain.add_block(genesis_block);
    let block = blockchain.new_block();
    blockchain.add_block(block);
    println!("{:?}", blockchain)
}
