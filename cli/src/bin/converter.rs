use clap::Parser;
use cli::ConverterCli;
use formats::bin_format::bin_parser;
use formats::csv_format::csv_parser;
use formats::text_format::text_parser;
use formats::{Format, ParseError, Record};
use std::fs::File;

fn main() -> Result<(), ParseError> {
    let cli = ConverterCli::parse();

    let from_file_name = cli.input;
    let from_format = cli.input_format;
    let to_file = cli.output;
    let to_format = cli.output_format;

    let data = get_transactions_from(&from_file_name, &from_format.into())?;

    write_transactions_to(&to_file, &to_format.into(), data)?;
    println!("Converted successfully");
    Ok(())
}

fn get_transactions_from(from_file: &str, from_format: &Format) -> Result<Vec<Record>, ParseError> {
    let f = File::open(from_file).map_err(ParseError::Io)?;

    match from_format {
        Format::Text => text_parser::read_from(f),
        Format::Csv => csv_parser::read_from(f),
        Format::Bin => bin_parser::read_from(f),
    }
}

fn write_transactions_to(
    to_file: &str,
    to_format: &Format,
    data: Vec<Record>,
) -> Result<(), ParseError> {
    let mut file = File::create(to_file).map_err(ParseError::Io)?;

    match to_format {
        Format::Text => text_parser::write_to(&mut file, data),
        Format::Csv => csv_parser::write_to(&mut file, data),
        Format::Bin => bin_parser::write_to(&mut file, data),
    }
}
