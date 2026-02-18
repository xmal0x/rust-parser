pub mod text_parser {
    use core::error::ParseError;
    use core::model::{Record, TextRecordDraft, TransactionStatus, TransactionType};
    use std::io::{self, BufRead, BufWriter, Write};

    /// Read transactions from text format and converting to Record entity
    ///
    /// # Examples
    ///
    /// ```
    /// let data = "# Record 1 (DEPOSIT)
    ///     TX_TYPE: DEPOSIT
    ///     TO_USER_ID: 9223372036854775807
    ///     FROM_USER_ID: 0
    ///     TIMESTAMP: 1633036860000
    ///     DESCRIPTION: \"Record number 1\"
    ///     TX_ID: 1000000000000000
    ///     AMOUNT: 100
    ///     STATUS: FAILURE";
    ///
    /// let cursor = std::io::Cursor::new(&data[..]);
    /// let r = formats::text_format::text_parser::read_from(cursor).unwrap();
    ///
    /// assert_eq!(r.len(), 1);
    /// ```
    pub fn read_from<R: std::io::Read>(r: R) -> Result<Vec<Record>, ParseError> {
        let reader = io::BufReader::new(r);
        let mut data: Vec<Record> = Vec::new();

        let mut draft = TextRecordDraft {
            tx_id: None,
            tx_type: None,
            to_user_id: None,
            from_user_id: None,
            amount: None,
            timestamp: None,
            description: None,
            status: None,
        };

        for line in reader.lines() {
            let line = line.map_err(ParseError::Io)?;
            let line = line.trim();

            if line.is_empty() && !draft.is_empty() {
                let record = Record::from_draft(&draft)?;
                data.push(record);
                draft.reset();
                continue;
            } else if line.is_empty() {
                continue;
            }

            if line.starts_with('#') {
                continue;
            }

            let mut it = line.splitn(2, ':').map(|s| s.trim());
            let key = it.next().ok_or(ParseError::MalformedLine)?;
            let value = it.next().ok_or(ParseError::MalformedLine)?;

            match key {
                "TX_ID" => draft.tx_id = Some(value.parse().unwrap()),
                "FROM_USER_ID" => draft.from_user_id = Some(parse_str(value)?),
                "TO_USER_ID" => draft.to_user_id = Some(parse_str(value)?),
                "TIMESTAMP" => draft.timestamp = Some(parse_str(value)?),
                "AMOUNT" => draft.amount = Some(parse_str(value)?),
                "TX_TYPE" => draft.tx_type = Some(TransactionType::parse(value)?),
                "DESCRIPTION" => draft.description = Some(value.to_string()),
                "STATUS" => draft.status = Some(TransactionStatus::parse(value)?),
                _ => return Err(ParseError::MalformedLine),
            }
        }
        if !draft.is_empty() {
            let record = Record::from_draft(&draft)?;
            data.push(record);
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
    /// let r = formats::text_format::text_parser::write_to(&mut cursor, mock);
    ///
    /// assert!(r.is_ok());
    ///
    /// cursor.set_position(0);
    /// let lines: Vec<String> = std::io::BufReader::new(cursor).lines().collect::<Result<_, _>>().unwrap();
    ///
    /// assert_eq!(lines.len(), 8);
    /// ```
    pub fn write_to<W: std::io::Write>(
        writer: &mut W,
        records: Vec<Record>,
    ) -> Result<(), ParseError> {
        let mut buffer = BufWriter::new(writer);
        let mut data = String::new();

        for record in records {
            let s = format!(
                "TX_ID: {}\nTX_TYPE: {}\nTO_USER_ID: {}\nFROM_USER_ID: {}\nTIMESTAMP: {}\nDESCRIPTION: {}\nAMOUNT: {}\nSTATUS: {}\n\n",
                record.tx_id,
                TransactionType::to_str(&record.tx_type),
                record.to_user_id,
                record.from_user_id,
                record.timestamp,
                record.description,
                record.amount,
                TransactionStatus::to_str(&record.status)
            );
            data.push_str(&s);
        }

        write!(buffer, "{}", data.trim())?;

        Ok(())
    }

    fn parse_str(value: &str) -> Result<u64, ParseError> {
        value.parse().map_err(|_| ParseError::InvalidNumber)
    }
}

#[cfg(test)]
mod tests {
    use core::model::{Record, TransactionStatus, TransactionType};
    use std::io::{BufRead, BufReader, Cursor};

    use super::*;

    #[test]
    fn test_load_data() {
        let data = "
            # Record 1 (DEPOSIT)
            TX_TYPE: DEPOSIT
            TO_USER_ID: 9223372036854775807
            FROM_USER_ID: 0
            TIMESTAMP: 1633036860000
            DESCRIPTION: \"Record number 1\"
            TX_ID: 1000000000000000
            AMOUNT: 100
            STATUS: FAILURE

            # Record 2 (TRANSFER)
            DESCRIPTION: \"Record number 2\"
            TIMESTAMP: 1633036920000
            STATUS: PENDING
            AMOUNT: 200
            TX_ID: 1000000000000001
            TX_TYPE: TRANSFER
            FROM_USER_ID: 9223372036854775807
            TO_USER_ID: 9223372036854775807

            # Record 3 (WITHDRAWAL)
            DESCRIPTION: \"Record number 3\"
            FROM_USER_ID: 599094029349995112
            TX_ID: 1000000000000002
            TO_USER_ID: 0
            AMOUNT: 300
            TX_TYPE: WITHDRAWAL
            STATUS: SUCCESS
            TIMESTAMP: 1633036980000
            ";

        let cursor = Cursor::new(&data[..]);
        let r = text_parser::read_from(cursor);

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

        let r = text_parser::write_to(&mut cursor, data);

        assert!(r.is_ok());
        cursor.set_position(0);
        let lines: Vec<String> = BufReader::new(cursor)
            .lines()
            .collect::<Result<_, _>>()
            .unwrap();

        assert_eq!(lines.len(), 26);
    }
}
