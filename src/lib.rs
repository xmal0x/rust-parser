use error::ParseTransactionTypeError;

use crate::error::ParseTransactionStatusError;

pub mod bin_format;
pub mod csv_format;
pub mod error;
pub mod text_format;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Record {
    pub tx_id: u64,
    pub tx_type: TransactionType,
    pub from_user_id: u64,
    pub to_user_id: u64,
    pub amount: u64,
    pub timestamp: u64,
    pub status: TransactionStatus,
    pub description: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

impl TransactionType {
    pub fn to_byte(&self) -> u8 {
        match self {
            TransactionType::Deposit => 0,
            TransactionType::Transfer => 1,
            TransactionType::Withdrawal => 2,
        }
    }

    pub fn from_byte(byte: u8) -> Result<TransactionType, ParseTransactionTypeError> {
        match byte {
            0 => Ok(TransactionType::Deposit),
            1 => Ok(TransactionType::Transfer),
            2 => Ok(TransactionType::Withdrawal),
            other => Err(ParseTransactionTypeError::UnknownTransactionTypeByte(other)),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            TransactionType::Deposit => "DEPOSIT",
            TransactionType::Transfer => "TRANSFER",
            TransactionType::Withdrawal => "WITHDRAWAL",
        }
    }

    pub fn parse(tx_type: &str) -> Result<TransactionType, ParseTransactionTypeError> {
        match tx_type {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "TRANSFER" => Ok(TransactionType::Transfer),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            other => Err(ParseTransactionTypeError::UnknownTransactionTypeString(
                other.to_string(),
            )),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum TransactionStatus {
    Success,
    Failure,
    Pending,
}

impl TransactionStatus {
    pub fn to_byte(&self) -> u8 {
        match self {
            TransactionStatus::Success => 0,
            TransactionStatus::Failure => 1,
            TransactionStatus::Pending => 2,
        }
    }

    pub fn from_byte(byte: u8) -> Result<TransactionStatus, ParseTransactionStatusError> {
        match byte {
            0 => Ok(TransactionStatus::Success),
            1 => Ok(TransactionStatus::Failure),
            2 => Ok(TransactionStatus::Pending),
            other => Err(ParseTransactionStatusError::UnknownTransactionStatusByte(
                other,
            )),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            TransactionStatus::Success => "SUCCESS",
            TransactionStatus::Failure => "FAILURE",
            TransactionStatus::Pending => "PENDING",
        }
    }

    pub fn parse(tx_type: &str) -> Result<TransactionStatus, ParseTransactionStatusError> {
        match tx_type {
            "SUCCESS" => Ok(TransactionStatus::Success),
            "FAILURE" => Ok(TransactionStatus::Failure),
            "PENDING" => Ok(TransactionStatus::Pending),
            other => Err(ParseTransactionStatusError::UnknownTransactionStatusString(
                other.to_string(),
            )),
        }
    }
}
