use std::sync::{Arc, Mutex};
use core::{Transactions, Transaction};

pub type SyncTransaction = Arc<Mutex<Transactions>>;

#[derive(Clone)]
pub struct TransactionPool {
    pub transactions: SyncTransaction,
}

impl TransactionPool {
    pub fn new() -> TransactionPool {
        TransactionPool {
            transactions: Arc::new(Mutex::new(Transactions::new())),
        }
    }

    pub fn add_transaction(&self, transaction: Transaction) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
    }

    pub fn clear_transactions(&self) -> Transactions {
        let mut transactions = self.transactions.lock().unwrap();
        let copy = transactions.clone();
        transactions.clear();
        return copy;
    }
}
