use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub enum TransactionError {
    InvalidTransaction,
    InvalidTransactionData,
    InvalidTransactionTimestamp,
    InvalidTransactionPublicKey,
    InvalidTransactionSignature,
    TransactionNotFound,
    TransactionIsEmpty,
    EmptyTransactionPool,
}

impl std::error::Error for TransactionError {
    fn description(&self) -> &str {
        match self {
            TransactionError::InvalidTransaction => "Invalid transaction",
            TransactionError::InvalidTransactionData => "Invalid transaction data",
            TransactionError::InvalidTransactionTimestamp => "Invalid transaction timestamp",
            TransactionError::InvalidTransactionPublicKey => "Invalid transaction public key",
            TransactionError::InvalidTransactionSignature => "Invalid transaction signature",
            TransactionError::TransactionNotFound => "Transaction not found",
            TransactionError::TransactionIsEmpty => "Transaction is empty",
            TransactionError::EmptyTransactionPool => "Transaction pool is empty",
        }
    }
}

impl Debug for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Display for TransactionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::InvalidTransaction => {
                write!(f, "Invalid transaction")
            }
            TransactionError::InvalidTransactionData => {
                write!(f, "Invalid transaction data")
            }
            TransactionError::InvalidTransactionTimestamp => {
                write!(f, "Invalid transaction timestamp")
            }
            TransactionError::InvalidTransactionPublicKey => {
                write!(f, "Invalid transaction public key")
            }
            TransactionError::InvalidTransactionSignature => {
                write!(f, "Invalid transaction signature")
            }
            TransactionError::TransactionNotFound => {
                write!(f, "Transaction not found")
            }
            TransactionError::TransactionIsEmpty => {
                write!(f, "Transaction is empty")
            }
            TransactionError::EmptyTransactionPool => {
                write!(f, "Not new pending transactions")
            }
        }
    }
}

