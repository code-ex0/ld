use context::Context;
use pool_blockchain::BlockchainPool;
use pool_sync::Run;
use pool_transactions::TransactionPool;


pub struct Api {
    pub blockchain: BlockchainPool,
    pub transactions: TransactionPool,
}

impl Api {
    pub fn new(context: &Context) -> Api {
        Api {
            blockchain: context.blockchain.clone(),
            transactions: context.transactions.clone(),
        }
    }

    pub fn start(&self) -> Result<(), String> {
        println!("Api is running...");
        loop {
            std::thread::sleep(std::time::Duration::from_secs(10));
            self.print_blockchain();
        }
    }

    pub fn print_blockchain(&self) {
        println!("{:?}", self.blockchain.blockchain);
    }

}


impl Run for Api {
    fn run(&self) -> Result<(), String> {
        self.start()
    }
}