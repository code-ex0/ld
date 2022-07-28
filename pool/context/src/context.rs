use pool_blockchain::BlockchainPool;
use pool_transactions::TransactionPool;
use config::Config;

pub struct Context {
    pub config: Config,
    pub transactions: TransactionPool,
    pub blockchain: BlockchainPool,
}
