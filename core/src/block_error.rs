use std::fmt::{Debug, Display, Formatter};

pub enum BlockError {
    InvalidHash,
    InvalidData,
    InvalidIndex,
    InvalidHashData,
    InvalidTimestamp,
    InvalidPreviousHash,
}

impl std::error::Error for BlockError {
    fn description(&self) -> &str {
        match self {
            BlockError::InvalidHash => "Invalid hash",
            BlockError::InvalidData => "Invalid data",
            BlockError::InvalidIndex => "Invalid index",
            BlockError::InvalidHashData => "Invalid hash data",
            BlockError::InvalidTimestamp => "Invalid timestamp",
            BlockError::InvalidPreviousHash => "Invalid previous hash",
        }
    }
}

impl Debug for BlockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}

impl Display for BlockError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BlockError::InvalidHash => write!(f, "Invalid hash"),
            BlockError::InvalidData => write!(f, "Invalid data"),
            BlockError::InvalidIndex => write!(f, "Invalid index"),
            BlockError::InvalidHashData => write!(f, "Invalid hash data"),
            BlockError::InvalidTimestamp => write!(f, "Invalid timestamp"),
            BlockError::InvalidPreviousHash => write!(f, "Invalid previous hash"),
        }
    }
}
