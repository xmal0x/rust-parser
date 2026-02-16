use error::ParseTransactionTypeError;

pub mod bin_format;
pub mod csv_format;
pub mod error;
pub mod text_format;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Record {
    pub tx_id: u64,
    tx_type: TransactionType,
    from_user_id: u64,
    to_user_id: u64,
    amount: i64,
    timestamp: u64,
    status: TransactionStatus,
    description: String,
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
            other @ _ => Err(ParseTransactionTypeError::UnknownTransactionTypeByte(other)),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            TransactionType::Deposit => "DEPOSIT",
            TransactionType::Transfer => "TRANSFER",
            TransactionType::Withdrawal => "WITHDRAWAL",
        }
    }

    pub fn from_str(tx_type: &str) -> Result<TransactionType, ParseTransactionTypeError> {
        match tx_type {
            "DEPOSIT" => Ok(TransactionType::Deposit),
            "TRANSFER" => Ok(TransactionType::Transfer),
            "WITHDRAWAL" => Ok(TransactionType::Withdrawal),
            _ => panic!("Unknown type"),
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

    pub fn from_byte(byte: u8) -> TransactionStatus {
        match byte {
            0 => TransactionStatus::Success,
            1 => TransactionStatus::Failure,
            2 => TransactionStatus::Pending,
            _ => panic!("Unknown type"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            TransactionStatus::Success => "SUCCESS",
            TransactionStatus::Failure => "FAILURE",
            TransactionStatus::Pending => "PENDING",
        }
    }

    pub fn from_str(tx_type: &str) -> TransactionStatus {
        match tx_type {
            "SUCCESS" => TransactionStatus::Success,
            "FAILURE" => TransactionStatus::Failure,
            "PENDING" => TransactionStatus::Pending,
            _ => panic!("Unknown type"),
        }
    }
}
