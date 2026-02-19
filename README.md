## Parser

support formats: 
- csv
- text
- bin

### Usage

**Converter**

Example

`cargo run -p cli --bin converter -- --input records_example.bin --input-format bin --output new.txt --output-format text`

**Comparer**

Example

`cargo run -p cli --bin comparer -- --file1 records_example.txt --format1 text --file2 records_example.csv --format2 cs`


### Contains

- `cli` crate with executed binaries comparer and converter
- `formats` crate contains logic for read and write varios formats
  - `bin_format` logic for read and write binary files
  - `text_format` logic for read and write text files
  - `csv_format` logic for read and write csv files
- `core` crate with core models
  - `error` errors
  - `model` main models
