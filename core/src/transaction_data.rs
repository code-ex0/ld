
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionData {
    Transfer {to: String, amount: u128},
    Create {receiver: String,amount: u128},
}