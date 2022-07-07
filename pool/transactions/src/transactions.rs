use std::sync::{Arc, Mutex};
use core::Payloads;

pub type SyncTransaction = Arc<Mutex<Payloads>>;

pub struct TransactionPool {
    pub transactions: SyncTransaction,
}

impl TransactionPool {
    pub fn new() -> TransactionPool {
        TransactionPool {
            transactions: Arc::new(Mutex::new(Payloads::new())),
        }
    }

    pub fn add_transaction(&self, transaction: Payload) {
        let mut transactions = self.transactions.lock().unwrap();
        transactions.push(transaction);
    }

    pub fn clear_transactions(&self) -> Payloads {
        let mut transactions = self.transactions.lock().unwrap();
        let copy = transactions.clone();
        transactions.clear();
        return copy;
    }
}
