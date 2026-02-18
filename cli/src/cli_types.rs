pub mod cli_types {
    use clap::Parser;
    use formats::model::Format;

    #[derive(Parser)]
    #[command(name = "Comparer")]
    #[command(version = "1.0")]
    #[command(about = "Compare transactions from 2 sources", long_about = None)]
    pub struct ComparerCli {
        #[arg(long)]
        pub file1: String,
        #[arg(long, value_enum)]
        pub format1: CliFormat,
        #[arg(long)]
        pub file2: String,
        #[arg(long, value_enum)]
        pub format2: CliFormat,
    }

    #[derive(Parser)]
    #[command(name = "Converter")]
    #[command(version = "1.0")]
    #[command(about = "Convert transactions from fromat to format", long_about = None)]
    pub struct ConverterCli {
        #[arg(long, value_name = "INPUT_FILE_NAME")]
        pub input: String,
        #[arg(long, short, value_enum)]
        pub input_format: CliFormat,
        #[arg(long, value_name = "OUTPUT_FILE_NAME")]
        pub output: String,
        #[arg(long, short, value_enum)]
        pub output_format: CliFormat,
    }

    #[derive(clap::ValueEnum, Clone, Debug)]
    pub enum CliFormat {
        Text,
        Csv,
        Bin,
    }

    impl From<CliFormat> for formats::model::Format {
        fn from(value: CliFormat) -> Self {
            match value {
                CliFormat::Bin => Format::Bin,
                CliFormat::Csv => Format::Csv,
                CliFormat::Text => Format::Text,
            }
        }
    }
}
