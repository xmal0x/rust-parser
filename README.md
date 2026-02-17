## Parser

support formats: 
- csv
- text
- bin

### Usage

**Converter**

`cargo run --bin converter records_example.bin bin foo3.txt text`

**Comparer**

`cargo run --bin comparer records_example.txt text records_example.csv csv`


### Contains

- `bin` folder for comparer and converter
- `bin_format` logic for read and write binary files
- `text_format` logic for read and write text files
- `csv_format` logic for read and write csv files
- `error` errors
- `lib` main library
