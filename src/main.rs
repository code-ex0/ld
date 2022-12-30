use config::Config;
use context::Context;
pub use core::{Block, Transaction, Blockchain};
pub use pool_blockchain::BlockchainPool;
use pool_sync::run_in_parallel;
use pool_miner::Miner;
use pool_transactions::TransactionPool;
use pool_api::Api;
use utils::set_ctrlc_handler;


fn main() {
    set_ctrlc_handler();

    let context = Context {
        config: Config {
            port: std::env::var("PORT").unwrap_or("8000".to_string()).parse().expect("Invalid port"),
            url: std::env::var("URL").unwrap_or("localhost".to_string()),
        },
        transactions: TransactionPool::new(),
        blockchain: BlockchainPool::new(),
    };

    let pool_miner = Miner::new(&context, "Louis Sasse".to_string());
    let pool_api = Api::new(&context);

    run_in_parallel(vec![&pool_miner, &pool_api]);
}
