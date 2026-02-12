#[derive(Debug)]
enum TransactionType {
    Deposit,
    Withdrawal,
    Transfer,
}

#[derive(Debug)]
enum TransactionStatus {
    Success,
    Failure,
    Pending,
}

// #[derive(Debug)]
// pub struct CsvRecord {
//     tx_id: u64,
//     tx_type: TransactionType,
//     from_user_id: u64,
//     to_user_id: u64,
//     amount: u64,
//     timestamp: i64,
//     status: TransactionStatus,
//     description: String,
// }
//
use crate::Record;

pub mod csv_parser {
    use crate::csv_format::{Record, TransactionStatus, TransactionType};
    use std::io::{self, BufRead, BufWriter, Error, Write};

    const HEADER: &str =
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

    pub fn read_from<R: std::io::Read>(r: R) -> Result<Vec<Record>, Error> {
        let mut reader = io::BufReader::new(r);
        let mut data: Vec<Record> = Vec::new();
        let mut header = String::new();
        let _ = reader.read_line(&mut header)?;
        println!("Header {}", header);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 8 {
                let tx_id: u64 = parts[0].parse().unwrap();
                let tx_type: String = parts[1].to_string();
                let from_user_id: u64 = parts[2].parse().unwrap();
                let to_user_id: u64 = parts[3].parse().unwrap();
                let amount: u64 = parts[4].parse().unwrap();
                let timestamp: i64 = parts[5].parse().unwrap();
                let status: String = parts[6].to_string();
                let description: String = parts[7].to_string();

                data.push(Record {
                    tx_id,
                    tx_type,
                    from_user_id,
                    to_user_id,
                    amount,
                    timestamp,
                    status,
                    description,
                });
            }
        }
        Ok(data)
    }

    pub fn write_to<W: std::io::Write>(writer: &mut W, records: Vec<Record>) -> Result<(), Error> {
        let mut buffer = BufWriter::new(writer);
        let mut data = String::from(format!("{}\n", HEADER));

        for record in records {
            data.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                record.tx_id,
                record.tx_type,
                record.from_user_id,
                record.to_user_id,
                record.amount,
                record.timestamp,
                record.status,
                record.description
            ));
        }

        write!(buffer, "{}", data)?;

        Ok(())
    }

    // fn parse_to_transaction_type(value: &str) -> TransactionType {
    //     match value {
    //         "WITHDRAWAL" => TransactionType::Withdrawal,
    //         "DEPOSIT" => TransactionType::Deposit,
    //         "TRANSFER" => TransactionType::Transfer,
    //         _ => panic!("Unknown transaction type"),
    //     }
    // }

    // fn parse_from_transaction_type(value: &TransactionType) -> &str {
    //     match value {
    //         TransactionType::Deposit => "DEPOSIT",
    //         TransactionType::Transfer => "TRANSFER",
    //         TransactionType::Withdrawal => "WITHDRAWAL",
    //     }
    // }

    // fn parse_to_transaction_status(value: &str) -> TransactionStatus {
    //     match value {
    //         "SUCCESS" => TransactionStatus::Success,
    //         "FAILURE" => TransactionStatus::Failure,
    //         "PENDING" => TransactionStatus::Pending,
    //         _ => panic!("Unknown transaction status"),
    //     }
    // }

    // fn parse_from_transaction_status(value: &TransactionStatus) -> &str {
    //     match value {
    //         TransactionStatus::Success => "SUCCESS",
    //         TransactionStatus::Failure => "FAILURE",
    //         TransactionStatus::Pending => "PENDING",
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use csv_parser;
    use std::io::{BufRead, BufReader, Cursor};

    use super::*;

    #[test]
    fn test_load_data() {
        let data = b"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n
            1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"\n
            1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"\n
        1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,SUCCESS,\"Record number 3\"";

        let cursor = Cursor::new(&data[..]);
        let r = csv_parser::read_from(cursor);

        assert_eq!(r.unwrap().len(), 3);
    }

    #[test]
    fn test_write_data() {
        let data = vec![
            Record {
                tx_id: 1000000000000000,
                tx_type: "DEPOSIT".to_string(),
                from_user_id: 0,
                to_user_id: 9223372036854775807,
                amount: 100,
                timestamp: 1633036860000,
                status: "FAILURE".to_string(),
                description: "Record number 1".to_string(),
            },
            Record {
                tx_id: 1000000000000001,
                tx_type: "TRANSFER".to_string(),
                from_user_id: 9223372036854775807,
                to_user_id: 9223372036854775807,
                amount: 200,
                timestamp: 1633036920000,
                status: "PENDING".to_string(),
                description: "Record number 2".to_string(),
            },
            Record {
                tx_id: 1000000000000002,
                tx_type: "WITHDRAWAL".to_string(),
                from_user_id: 599094029349995112,
                to_user_id: 0,
                amount: 300,
                timestamp: 1633036980000,
                status: "SUCCESS".to_string(),
                description: "Record number 3".to_string(),
            },
        ];

        let mut cursor = Cursor::new(Vec::new());

        let r = csv_parser::write_to(&mut cursor, data);

        assert!(r.is_ok());
        cursor.set_position(0);
        let lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();

        assert_eq!(lines.len(), 4);
    }
}
