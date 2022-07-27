use config::Config;
use context::Context;
pub use core::{Block, Transaction, Blockchain};
pub use pool_blockchain::BlockchainPool;
use pool_sync::run_in_parallel;
use pool_miner::Miner;
use pool_transactions::TransactionPool;
use pool_api::Api;


fn main() {
    let context = Context {
        config: Config {
            port: 8000
        },
        transactions: TransactionPool::new(),
        blockchain: BlockchainPool::new(),
    };


    let pool_miner = Miner::new(&context, "Louis Sasse".to_string());
    let pool_api = Api::new(&context);

    run_in_parallel(vec![&pool_miner, &pool_api]);
}
