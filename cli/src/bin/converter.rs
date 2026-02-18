use clap::Parser;
use formats::bin_format::bin_parser;
use formats::csv_format::csv_parser;
use formats::error::ParseError;
use formats::model::Record;
use formats::text_format::text_parser;
use std::fs::File;

#[derive(Parser)]
#[command(name = "Converter")]
#[command(version = "1.0")]
#[command(about = "Convert transactions from fromat to format", long_about = None)]
struct Cli {
    #[arg(long, value_name = "INPUT_FILE_NAME")]
    input: String,
    #[arg(long, short)]
    input_format: String,
    #[arg(long, value_name = "OUTPUT_FILE_NAME")]
    output: String,
    #[arg(long, short)]
    output_format: String,
}

fn main() -> Result<(), ParseError> {
    let cli = Cli::parse();

    let from_file_name = cli.input;
    let from_format = cli.input_format;
    let to_file = cli.output;
    let to_format = cli.output_format;

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
