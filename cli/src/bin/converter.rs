use formats::bin_format::bin_parser;
use formats::csv_format::csv_parser;
use formats::error::ParseError;
use formats::model::Record;
use formats::text_format::text_parser;
use std::{env, fs::File};

fn main() -> Result<(), ParseError> {
    let mut args = env::args().skip(1);

    const USAGE_MSG: &str = "Usage: text_file.txt text csv_file.csv csv";

    let from_file_name = args.next().ok_or(ParseError::InvalidArgument(USAGE_MSG))?;
    let from_format = args.next().ok_or(ParseError::InvalidArgument(USAGE_MSG))?;
    let to_file = args.next().ok_or(ParseError::InvalidArgument(USAGE_MSG))?;
    let to_format = args.next().ok_or(ParseError::InvalidArgument(USAGE_MSG))?;

    let data = get_transactions_from(&from_file_name, &from_format)?;

    write_transactions_to(&to_file, &to_format, data)?;
    println!("Converted successfully");
    Ok(())
}

fn get_transactions_from(from_file: &str, from_format: &str) -> Result<Vec<Record>, ParseError> {
    let f = File::open(from_file).map_err(ParseError::Io)?;

    match from_format {
        "text" => text_parser::read_from(f),
        "csv" => csv_parser::read_from(f),
        "bin" => bin_parser::read_from(f),
        other => Err(ParseError::InvalidFormat(other.to_string())),
    }
}

fn write_transactions_to(
    to_file: &str,
    to_format: &str,
    data: Vec<Record>,
) -> Result<(), ParseError> {
    let mut file = File::create(to_file).map_err(ParseError::Io)?;

    match to_format {
        "text" => text_parser::write_to(&mut file, data),
        "csv" => csv_parser::write_to(&mut file, data),
        "bin" => bin_parser::write_to(&mut file, data),
        other => Err(ParseError::InvalidFormat(other.to_string())),
    }
}
