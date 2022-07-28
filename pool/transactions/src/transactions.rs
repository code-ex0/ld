use std::sync::{Arc, Mutex};
use core::{Transactions, Transaction, TransactionError};

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

    pub fn get_all_transactions(&self) -> Transactions {
        let transactions = self.transactions.lock().unwrap();
        return transactions.clone();
    }

    pub fn get_transaction(&self, id: u64) -> Result<Transaction, TransactionError> {
        let transactions = self.transactions.lock().unwrap();
        if transactions.is_empty() {
            return Err(TransactionError::EmptyTransactionPool)?;
        }
        match transactions.iter().enumerate().find(|(current_id, transaction)| current_id == id.try_into().as_ref().unwrap()) {
            Some((_, transaction)) => Ok(transaction.clone()),
            None => Err(TransactionError::TransactionNotFound)?,
        }
    }
}
