use rust_parser::csv_format::csv_parser;
use rust_parser::text_format::text_parser;
use std::collections::HashMap;
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 5 {
        println!("Example: text_file.txt text csv_file.csv csv");
        return;
    }

    let mut transactions = HashMap::new();

    let file_name_1: &str = &args[1];
    let format_1: &str = &args[2];
    let file_name_2: &str = &args[3];
    let format_2: &str = &args[4];

    let file_1 = File::open(file_name_1).unwrap();

    let transactions_1 = match format_1 {
        "text" => text_parser::read_from(file_1),
        "csv" => csv_parser::read_from(file_1),
        _ => panic!("Unknown format"),
    };

    for t in transactions_1.unwrap() {
        transactions.insert(t.tx_id, 1);
    }

    let file_2 = File::open(file_name_2).unwrap();

    let transactions_2 = match format_2 {
        "text" => text_parser::read_from(file_2),
        "csv" => csv_parser::read_from(file_2),
        _ => panic!("Unknown format"),
    };

    for t in transactions_2.unwrap() {
        if transactions.contains_key(&t.tx_id) {
            let _ = transactions.remove(&t.tx_id);
        }
    }

    if transactions.is_empty() {
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
