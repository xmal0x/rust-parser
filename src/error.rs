use std::error::Error;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub enum ParseError {
    TransactionType(ParseTransactionTypeError),
    Io(io::Error),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TransactionType(e) => write!(f, "Transaction type error: {}", e),
            Self::Io(e) => write!(f, "Io error: {}", e),
        }
    }
}

impl Error for ParseError {}

impl From<ParseTransactionTypeError> for ParseError {
    fn from(value: ParseTransactionTypeError) -> Self {
        ParseError::TransactionType(value)
    }
}

impl From<io::Error> for ParseError {
    fn from(value: io::Error) -> Self {
        ParseError::Io(value)
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

impl Error for ParseTransactionTypeError {}
