use std::{env, fs::File};

use rust_parser::csv_format::csv_parser;
use rust_parser::text_format::text_parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 5 {
        println!("Example: text_file.txt text csv_file.csv csv");
        return;
    }

    let from_file_name: &str = &args[1];
    let from_format: &str = &args[2];
    let to_file: &str = &args[3];
    let to_format: &str = &args[4];

    let f = File::open(from_file_name).unwrap();

    let data = match from_format {
        "text" => text_parser::read_from(f),
        "csv" => csv_parser::read_from(f),
        _ => panic!("Unknown format"),
    };

    let mut buffer = File::create(to_file).unwrap();

    let r = match to_format {
        "text" => text_parser::write_to(&mut buffer, data.unwrap()),
        "csv" => csv_parser::write_to(&mut buffer, data.unwrap()),
        _ => panic!("Unknown format"),
    };

    assert!(r.is_ok());
}
