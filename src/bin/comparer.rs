use rust_parser::Record;
use rust_parser::bin_format::bin_parser;
use rust_parser::csv_format::csv_parser;
use rust_parser::text_format::text_parser;
use std::collections::HashSet;
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 5 {
        println!("Example: text_file.txt text csv_file.csv csv");
        return;
    }

    let file_name_1: &str = &args[1];
    let format_1: &str = &args[2];
    let file_name_2: &str = &args[3];
    let format_2: &str = &args[4];

    let file_1 = match File::open(file_name_1) {
        Ok(f) => f,
        Err(e) => {
            println!("Can not read file {} {}", file_name_1, e);
            return;
        }
    };

    let transactions_1 = match format_1 {
        "text" => text_parser::read_from(file_1),
        "csv" => csv_parser::read_from(file_1),
        "bin" => bin_parser::read_from(file_1),
        _ => {
            println!("Unknown format {}", format_1);
            return;
        }
    };

    let transactions_1 = match transactions_1 {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error {}", e);
            return;
        }
    };

    let file_2 = match File::open(file_name_2) {
        Ok(f) => f,
        Err(e) => {
            println!("Can not read file {} {}", file_name_2, e);
            return;
        }
    };

    let transactions_2 = match format_2 {
        "text" => text_parser::read_from(file_2),
        "csv" => csv_parser::read_from(file_2),
        "bin" => bin_parser::read_from(file_2),
        _ => {
            println!("Unknown format {}", format_2);
            return;
        }
    };

    let transactions_2 = match transactions_2 {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error {}", e);
            return;
        }
    };

    if is_equal_transactions(&transactions_1, &transactions_2) {
        println!(
            "The transaction records in '{}' and '{}' are identical.",
            file_name_1, file_name_2
        )
    } else {
        println!(
            "The transaction records in '{}' and '{}' are not identical.",
            file_name_1, file_name_2
        )
    }
}

fn is_equal_transactions(transactions_1: &[Record], transactions_2: &[Record]) -> bool {
    let mut transactions = HashSet::new();

    for t in transactions_1 {
        transactions.insert(t);
    }

    for t in transactions_2 {
        if transactions.contains(&t) {
            let _ = transactions.remove(&t);
        } else {
            // if not contains then transactions not identical
            return false;
        }
    }

    if !transactions.is_empty() {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_parser::{TransactionStatus, TransactionType};

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
    fn records_mock_3() -> [Record; 2] {
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
        let is_equal = is_equal_transactions(&records_mock_1(), &records_mock_2());

        assert!(is_equal);
    }

    #[test]
    fn test_not_equal_transactions() {
        let is_equal = is_equal_transactions(&records_mock_2(), &records_mock_3());

        assert!(!is_equal);
    }
}
