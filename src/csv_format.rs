pub mod csv_parser {
    use crate::{Record, TransactionStatus, TransactionType, error::ParseError};
    use std::io::{self, BufRead, BufWriter, Error, Write};

    const HEADER: &str =
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

    pub fn read_from<R: std::io::Read>(r: R) -> Result<Vec<Record>, ParseError> {
        let mut reader = io::BufReader::new(r);
        let mut data: Vec<Record> = Vec::new();
        let mut header = String::new();
        let _ = reader.read_line(&mut header)?;
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 8 {
                let tx_id: u64 = parts[0].parse().unwrap();
                let tx_type: TransactionType = TransactionType::from_str(parts[1])?;
                let from_user_id: u64 = parts[2].parse().unwrap();
                let to_user_id: u64 = parts[3].parse().unwrap();
                let amount: i64 = parts[4].parse().unwrap();
                let timestamp: u64 = parts[5].parse().unwrap();
                let status: TransactionStatus = TransactionStatus::from_str(parts[6]);
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
                TransactionType::to_str(&record.tx_type),
                record.from_user_id,
                record.to_user_id,
                record.amount,
                record.timestamp,
                TransactionStatus::to_str(&record.status),
                record.description
            ));
        }

        write!(buffer, "{}", data)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Record, TransactionStatus, TransactionType};
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
                tx_type: TransactionType::Deposit,
                from_user_id: 0,
                to_user_id: 9223372036854775807,
                amount: 100,
                timestamp: 1633036860000,
                status: TransactionStatus::Failure,
                description: "Record number 1".to_string(),
            },
            Record {
                tx_id: 1000000000000001,
                tx_type: TransactionType::Transfer,
                from_user_id: 9223372036854775807,
                to_user_id: 9223372036854775807,
                amount: 200,
                timestamp: 1633036920000,
                status: TransactionStatus::Pending,
                description: "Record number 2".to_string(),
            },
            Record {
                tx_id: 1000000000000002,
                tx_type: TransactionType::Withdrawal,
                from_user_id: 599094029349995112,
                to_user_id: 0,
                amount: 300,
                timestamp: 1633036980000,
                status: TransactionStatus::Success,
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
