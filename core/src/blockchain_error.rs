use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use crate::BlockError;

pub enum BlockchainError {
    EmptyBlockchain,
    BlockNotFound,
    InvalidBlock(BlockError),
}

impl std::error::Error for BlockchainError {
    fn description(&self) -> &str {
        match self {
            BlockchainError::EmptyBlockchain => "Blockchain is empty",
            BlockchainError::InvalidBlock(block_error) => block_error.description(),
            BlockchainError::BlockNotFound => "Block not found",
        }
    }
}

impl Debug for BlockchainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl Display for BlockchainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockchainError::EmptyBlockchain => {
                write!(f, "Blockchain is empty")
            }
            BlockchainError::InvalidBlock(block_error) => {
                write!(f, "{}", block_error.description())
            }
            BlockchainError::BlockNotFound => {
                write!(f, "Block not found")
            }
        }
    }
}

