use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    TransactionType(ParseTransactionTypeError),
    TransactionStatus(ParseTransactionStatusError),
    RecordDamaged(u64),
    UnexpectedEof { needed: usize, got: usize },
    UnexpectedRecordSize(u32),
    InvalidMagic,
    RecordTooShort,
    Io(io::Error),
    InvalidUtf8(std::string::FromUtf8Error),
    InvalidNumber,
    MalformedLine,
    MissingField(&'static str),
    InvalidArgument(&'static str),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TransactionType(e) => write!(f, "Transaction type error: {}", e),
            Self::TransactionStatus(e) => write!(f, "Transaction status error: {}", e),
            Self::RecordDamaged(tx_id) => write!(f, "Record with id '{}' damaged", tx_id),
            Self::UnexpectedEof { needed, got } => {
                write!(f, "Unexpected Eof, needed {}, got {}", needed, got)
            }
            Self::UnexpectedRecordSize(s) => {
                write!(f, "Unexpected Record size {}", s)
            }
            Self::InvalidMagic => write!(f, "Invalid magic"),
            Self::RecordTooShort => write!(f, "Record too short"),
            Self::Io(e) => write!(f, "Io error: {}", e),
            Self::InvalidUtf8(e) => write!(f, "Invalid Utf-8 format {}", e),
            Self::InvalidNumber => write!(f, "Invalid number"),
            Self::MalformedLine => write!(f, "Malformed line"),
            Self::MissingField(field) => write!(f, "Missing field {}", field),
            Self::InvalidArgument(message) => write!(f, "Invalid argument: {}", message),
        }
    }
}

impl Error for ParseError {}

impl From<ParseTransactionTypeError> for ParseError {
    fn from(value: ParseTransactionTypeError) -> Self {
        ParseError::TransactionType(value)
    }
}

impl From<ParseTransactionStatusError> for ParseError {
    fn from(value: ParseTransactionStatusError) -> Self {
        ParseError::TransactionStatus(value)
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::Io(value)
    }
}

impl From<std::string::FromUtf8Error> for ParseError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        ParseError::InvalidUtf8(value)
    }
}

#[derive(Debug)]
pub enum ParseTransactionTypeError {
    UnknownTransactionTypeByte(u8),
    UnknownTransactionTypeString(String),
}

impl Display for ParseTransactionTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTransactionTypeError::UnknownTransactionTypeByte(b) => {
                write!(f, "Unknown transaction type '{}'", b)
            }
            ParseTransactionTypeError::UnknownTransactionTypeString(s) => {
                write!(f, "Unknown transaction type '{}'", s)
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseTransactionStatusError {
    UnknownTransactionStatusByte(u8),
    UnknownTransactionStatusString(String),
}

impl Display for ParseTransactionStatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTransactionStatusError::UnknownTransactionStatusByte(b) => {
                write!(f, "Unknown transaction status '{}'", b)
            }
            ParseTransactionStatusError::UnknownTransactionStatusString(s) => {
                write!(f, "Unknown transaction status '{}'", s)
            }
        }
    }
}
