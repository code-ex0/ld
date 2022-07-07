pub use core::{Block, Payload, Blockchain};

fn main() {
    let mut blockchain = Blockchain::new();


    let block = Block::new(0, 0, Vec::new(), "".to_string(), 0);
    for _i in 0..10 {
        blockchain.add_block(block.clone());
    }

    blockchain.into_iter().for_each(|mut block| {
        println!("{:?}", block);
        block.id += 1;
    });

    blockchain.into_iter().for_each(|block| {
        println!("{:?}", block);
    });
}
