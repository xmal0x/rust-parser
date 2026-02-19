use std::{fmt::Display, fs::File};

use crate::error::{ParseError, ParseTransactionStatusError, ParseTransactionTypeError};

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

impl Record {
    pub fn from_draft(draft: &TextRecordDraft) -> Result<Record, ParseError> {
        Ok(Record {
            tx_id: draft.tx_id.ok_or(ParseError::MissingField("tx_id"))?,
            from_user_id: draft
                .from_user_id
                .ok_or(ParseError::MissingField("from_user_id"))?,
            to_user_id: draft
                .to_user_id
                .ok_or(ParseError::MissingField("to_user_id"))?,
            amount: draft.amount.ok_or(ParseError::MissingField("amount"))?,
            tx_type: draft
                .tx_type
                .clone()
                .ok_or(ParseError::MissingField("tx_type"))?,
            timestamp: draft
                .timestamp
                .ok_or(ParseError::MissingField("timestamp"))?,
            status: draft
                .status
                .clone()
                .ok_or(ParseError::MissingField("status"))?,
            description: draft
                .description
                .clone()
                .ok_or(ParseError::MissingField("description"))?,
        })
    }
}

#[derive(Debug)]
pub struct TextRecordDraft {
    pub tx_id: Option<u64>,
    pub tx_type: Option<TransactionType>,
    pub from_user_id: Option<u64>,
    pub to_user_id: Option<u64>,
    pub amount: Option<u64>,
    pub timestamp: Option<u64>,
    pub status: Option<TransactionStatus>,
    pub description: Option<String>,
}

impl TextRecordDraft {
    pub fn reset(&mut self) {
        self.tx_id = None;
        self.tx_type = None;
        self.from_user_id = None;
        self.to_user_id = None;
        self.amount = None;
        self.timestamp = None;
        self.status = None;
        self.description = None;
    }

    pub fn is_empty(&self) -> bool {
        self.tx_id.is_none()
            && self.tx_type.is_none()
            && self.from_user_id.is_none()
            && self.to_user_id.is_none()
            && self.amount.is_none()
            && self.timestamp.is_none()
            && self.status.is_none()
            && self.description.is_none()
    }
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

#[derive(Clone, Debug)]
pub enum Format {
    Csv,
    Text,
    Bin,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Csv => write!(f, "Csv"),
            Format::Text => write!(f, "Text"),
            Format::Bin => write!(f, "Bin"),
        }
    }
}

pub trait Reader {
    fn read_from(file: File) -> Result<Vec<Record>, ParseError>;
}
