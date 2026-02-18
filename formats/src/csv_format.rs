pub mod csv_parser {
    use core::{
        error::ParseError,
        model::{Record, TransactionStatus, TransactionType},
    };
    use std::io::{self, BufRead, BufWriter, Write};

    const HEADER: &str =
        "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION";

    /// Read transactions from csv format and converting to Record entity
    ///
    /// # Examples
    ///
    /// ```
    /// let data = b"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"\n";
    ///
    /// let cursor = std::io::Cursor::new(&data[..]);
    /// let r = formats::csv_format::csv_parser::read_from(cursor).unwrap();
    ///
    /// assert_eq!(r.len(), 1);
    /// ```
    pub fn read_from<R: std::io::Read>(r: R) -> Result<Vec<Record>, ParseError> {
        let mut reader = io::BufReader::new(r);
        let mut data: Vec<Record> = Vec::new();
        let mut header = String::new();
        reader.read_line(&mut header)?;

        for line in reader.lines() {
            let line = line.map_err(|_| ParseError::MalformedLine)?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let mut it = line.split(',');
            let tx_id = self::parse_number(it.next())?;

            let tx_type_value = it.next().ok_or(ParseError::MalformedLine)?;
            let tx_type: TransactionType = TransactionType::parse(tx_type_value)?;
            let from_user_id = self::parse_number(it.next())?;
            let to_user_id = self::parse_number(it.next())?;
            let amount = self::parse_number(it.next())?;
            let timestamp = self::parse_number(it.next())?;
            let status_value = it.next().ok_or(ParseError::MalformedLine)?;
            let status: TransactionStatus = TransactionStatus::parse(status_value)?;
            let description: String = it.next().ok_or(ParseError::MalformedLine)?.to_string();

            if it.next().is_some() {
                return Err(ParseError::MalformedLine);
            }

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
        Ok(data)
    }

    /// Write transactions of Record entity to csv format
    ///
    /// # Examples
    ///
    /// ```
    /// use std::io::{BufRead, BufReader};
    ///
    /// let mock: Vec<core::model::Record> = vec![
    /// core::model::Record {
    ///     tx_id: 1000000000000000,
    ///     tx_type: core::model::TransactionType::Deposit,
    ///     from_user_id: 0,
    ///     to_user_id: 9223372036854775807,
    ///     amount: 100,
    ///     timestamp: 1633036860000,
    ///     status: core::model::TransactionStatus::Failure,
    ///     description: String::from("\"Record number 1\""),
    /// }];
    ///
    /// let mut cursor = std::io::Cursor::new(Vec::new());
    /// let r = formats::csv_format::csv_parser::write_to(&mut cursor, mock);
    ///
    /// assert!(r.is_ok());
    ///
    /// cursor.set_position(0);
    /// let lines: Vec<String> = std::io::BufReader::new(cursor).lines().collect::<Result<_, _>>().unwrap();
    ///
    /// assert_eq!(lines.len(), 2);
    /// ```
    pub fn write_to<W: std::io::Write>(
        writer: &mut W,
        records: Vec<Record>,
    ) -> Result<(), ParseError> {
        let mut buffer = BufWriter::new(writer);
        let mut data = format!("{}\n", HEADER);

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

    fn parse_number(value: Option<&str>) -> Result<u64, ParseError> {
        let str = value.ok_or(ParseError::MalformedLine)?;
        let result = str.parse::<u64>().map_err(|_| ParseError::InvalidNumber)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use core::model::{Record, TransactionStatus, TransactionType};
    use std::io::{BufRead, BufReader, Cursor};

    use super::*;

    #[test]
    fn test_load_data() {
        let data = b"TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n1000000000000000,DEPOSIT,0,9223372036854775807,100,1633036860000,FAILURE,\"Record number 1\"\n1000000000000001,TRANSFER,9223372036854775807,9223372036854775807,200,1633036920000,PENDING,\"Record number 2\"\n1000000000000002,WITHDRAWAL,599094029349995112,0,300,1633036980000,SUCCESS,\"Record number 3\"";

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
        let lines: Vec<String> = BufReader::new(cursor)
            .lines()
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(lines.len(), 4);
    }
}
