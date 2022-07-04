pub use core::{Block, Payload, Blockchain};

fn main() {
    let mut blockchain = Blockchain::new();
    let block = Block::new(0, 0, Vec::new(), "".to_string(), 0);
    blockchain.add_block(block);
    println!("{:?}", blockchain);
    println!("Hello, world!");
}
