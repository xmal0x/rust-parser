use std::{env, fs::File};

use rust_parser::bin_format::bin_parser;
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

    let f = match File::open(from_file_name) {
        Ok(f) => f,
        Err(e) => {
            println!("Can not read file {} {}", from_file_name, e);
            return;
        }
    };

    let data = match from_format {
        "text" => text_parser::read_from(f),
        "csv" => csv_parser::read_from(f),
        "bin" => bin_parser::read_from(f),
        _ => {
            println!("Unknown format {}", from_format);
            return;
        }
    };

    let data = match data {
        Ok(d) => d,
        Err(e) => {
            println!("Parsing error {}", e);
            return;
        }
    };

    let mut buffer = match File::create(to_file) {
        Ok(f) => f,
        Err(e) => {
            println!("Can not create file {} {}", to_file, e);
            return;
        }
    };

    let r = match to_format {
        "text" => text_parser::write_to(&mut buffer, data),
        "csv" => csv_parser::write_to(&mut buffer, data),
        "bin" => bin_parser::write_to(&mut buffer, data),
        _ => {
            println!("Unknown format {}", from_format);
            return;
        }
    };

    assert!(r.is_ok());
}
