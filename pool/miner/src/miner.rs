use context::Context;
use pool_blockchain::BlockchainPool;
use pool_transactions::TransactionPool;
use pool_sync::Run;
use core::{Block, Transaction, Transactions, TransactionData};
use core::ProofOfWork;
use anyhow::Result;

pub struct Miner {
    pub address: String,
    pub blockchain: BlockchainPool,
    pub transactions: TransactionPool,
}

impl Miner {

    pub fn new(context: &Context, address: String) -> Miner {
        Miner {
            address,
            blockchain: context.blockchain.clone(),
            transactions: context.transactions.clone(),
        }
    }

    pub fn start(&self) -> Result<()> {
        println!("Miner is mining...");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
            println!("Miner is mining...");
            match self.blockchain.is_valid() {
                true => {
                    let mut block = self.new_block();
                    self.mine(&mut block);
                    self.blockchain.add_block(block.clone());
                }
                false => {
                    println!("Blockchain is not valid need to mine the current block");
                }
            }
        }
    }

    pub fn new_block(&self) -> Block {
        let transaction: Transaction = Transaction::new(
            1,
            TransactionData::Create { receiver: self.address.clone(), amount: 1000 },
            0,
            vec![],
            vec![],
        );
        let mut transactions: Transactions = vec![transaction];
        transactions.extend(self.transactions.clear_transactions());
        self.blockchain.new_block(transactions)
    }


    pub fn mine(&self, block: &mut Block) {
        block.mine(4);
    }

    pub fn add_block(&self, block: Block) {
        self.blockchain.add_block(block);
    }

    pub fn print_blockchain(&self) {
        println!("{:?}", self.blockchain.blockchain);
    }
}

impl Run for Miner {
    fn run(&self) -> Result<()> {
        self.start()
    }
}
