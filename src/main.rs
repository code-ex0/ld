pub use core::{Block, Payload, Blockchain};
pub use pool_blockchain::BlockchainPool;
use core::ProofOfWork;

fn main() {
    let blockchain = BlockchainPool::new();
    // let genesis_block = Block::new(0, 0, vec![], "genesis".to_string(), 0);
    let mut genesis = blockchain.new_block();
    genesis.mine(2);
    blockchain.add_block(genesis);
    let mut block = blockchain.new_block();
    block.mine(6);
    blockchain.add_block(block);
    println!("{:?}", blockchain);
}
