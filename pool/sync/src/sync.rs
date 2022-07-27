use pool_blockchain::BlockchainPool;
use pool_transactions::TransactionPool;


pub struct SyncPool {
    pub transactions: TransactionPool,
    pub blockchain: BlockchainPool,
}

impl SyncPool {
    pub fn new() -> SyncPool {
        SyncPool {
            transactions: TransactionPool::new(),
            blockchain: BlockchainPool::new(),
        }
    }
}