use clap::Parser;
use cli::cli_types::cli_types::ComparerCli;
use formats::bin_format::bin_parser::Bin;
use formats::csv_format::csv_parser::Csv;
use formats::error::ParseError;
use formats::model::{Format, Reader, Record};
use formats::text_format::text_parser::Text;
use std::collections::HashSet;
use std::fs::File;

fn main() -> Result<(), ParseError> {
    let cli = ComparerCli::parse();

    let file_name_1 = cli.file1;
    let format_1 = cli.format1;
    let file_name_2 = cli.file2;
    let format_2 = cli.format2;

    let transactions_1 = get_transactions_from_file(&file_name_1, &format_1.into())?;
    let transactions_2 = get_transactions_from_file(&file_name_2, &format_2.into())?;

    if is_equal_transactions(&transactions_1, &transactions_2) {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            file_name_1, file_name_2
        )
    } else {
        println!(
            "The transaction records in '{}' and '{}' are NOT identical.",
            file_name_1, file_name_2
        )
    }

    Ok(())
}

fn get_transactions_from_file(name: &str, format: &Format) -> Result<Vec<Record>, ParseError> {
    let file = File::open(name).map_err(ParseError::Io)?;

    match format {
        Format::Text => Text::read_from(file),
        Format::Csv => Csv::read_from(file),
        Format::Bin => Bin::read_from(file),
    }
}

fn is_equal_transactions(transactions_1: &[Record], transactions_2: &[Record]) -> bool {
    let mut transactions = HashSet::new();
    transactions.extend(transactions_1);

    for t in transactions_2 {
        if !transactions.remove(&t) {
            // if not contains then transactions not identical
            return false;
        }
    }

    transactions.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    use formats::model::{TransactionStatus, TransactionType};

    fn records_mock_1() -> [Record; 2] {
        [
            Record {
                tx_id: 1000000000000000,
                tx_type: TransactionType::Deposit,
                from_user_id: 0,
                to_user_id: 9223372036854775807,
                amount: 100,
                timestamp: 1633036860000,
                status: TransactionStatus::Failure,
                description: "\"Record number 1\"".to_string(),
            },
            Record {
                tx_id: 1000000000000001,
                tx_type: TransactionType::Transfer,
                from_user_id: 9223372036854775807,
                to_user_id: 9223372036854775807,
                amount: 200,
                timestamp: 1633036920000,
                status: TransactionStatus::Pending,
                description: "\"Record number 2\"".to_string(),
            },
        ]
    }

    // Different
    fn records_mock_2() -> [Record; 2] {
        [
            Record {
                tx_id: 1000000000000000,
                tx_type: TransactionType::Deposit,
                from_user_id: 0,
                to_user_id: 9223372036854775807,
                amount: 100,
                timestamp: 1633036860000,
                status: TransactionStatus::Failure,
                description: "\"Record number 1\"".to_string(),
            },
            Record {
                tx_id: 1000000000000002,
                tx_type: TransactionType::Withdrawal,
                from_user_id: 599094029349995112,
                to_user_id: 0,
                amount: 300,
                timestamp: 1633036980000,
                status: TransactionStatus::Success,
                description: "\"Record number 3\"".to_string(),
            },
        ]
    }

    #[test]
    fn test_equal_transactions() {
        let is_equal = is_equal_transactions(&records_mock_1(), &records_mock_1());

        assert!(is_equal);
    }

    #[test]
    fn test_not_equal_transactions() {
        let is_equal = is_equal_transactions(&records_mock_1(), &records_mock_2());

        assert!(!is_equal);
    }
}
