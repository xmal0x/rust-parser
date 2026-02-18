## Parser

support formats: 
- csv
- text
- bin

### Usage

**Converter**

`cargo run -p cli --bin converter records_example.bin bin foo3.txt text`

**Comparer**

`cargo run -p cli --bin comparer records_example.txt text records_example.csv csv`


### Contains

- `cli` crate with executed binaries comparer and converter
- `formats` crate contains logic for read and write varios formats
  - `bin_format` logic for read and write binary files
  - `text_format` logic for read and write text files
  - `csv_format` logic for read and write csv files
- `core` crate with core models
  - `error` errors
  - `model` main models
