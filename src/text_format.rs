use crate::{Record, TransactionStatus, TransactionType};

impl Record {
    // TODO add safety bulder
    fn from_draft(draft: &TextRecordDraft) -> Record {
        Record {
            tx_id: draft.tx_id.unwrap(),
            from_user_id: draft.from_user_id.unwrap(),
            to_user_id: draft.to_user_id.unwrap(),
            amount: draft.amount.unwrap(),
            tx_type: draft.tx_type.clone().unwrap(),
            timestamp: draft.timestamp.unwrap(),
            status: draft.status.clone().unwrap(),
            description: draft.description.clone().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct TextRecordDraft {
    tx_id: Option<u64>,
    tx_type: Option<TransactionType>,
    from_user_id: Option<u64>,
    to_user_id: Option<u64>,
    amount: Option<i64>,
    timestamp: Option<u64>,
    status: Option<TransactionStatus>,
    description: Option<String>,
}

impl TextRecordDraft {
    fn reset(&mut self) {
        self.tx_id = None;
        self.tx_type = None;
        self.from_user_id = None;
        self.to_user_id = None;
        self.amount = None;
        self.timestamp = None;
        self.status = None;
        self.description = None;
    }

    fn is_empty(&self) -> bool {
        self.tx_id.is_none()
            && self.tx_type.is_none()
            && self.from_user_id.is_none()
            && self.to_user_id.is_none()
            && self.amount.is_none()
            && self.timestamp.is_none()
            && self.status.is_none()
            && self.description.is_none()
    }
}

pub mod text_parser {
    use crate::error::ParseError;
    use crate::text_format::TextRecordDraft;
    use crate::{Record, TransactionStatus, TransactionType};
    use std::io::{self, BufRead, BufWriter, Error, Write};

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
            let line = line.unwrap();
            let line = line.trim();

            if line.is_empty() && !draft.is_empty() {
                let record = Record::from_draft(&draft);
                data.push(record);
                draft.reset();
                continue;
            } else if line.is_empty() {
                continue;
            }

            if line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split(':').map(|v| v.trim()).collect();

            match parts[0] {
                "TX_ID" => draft.tx_id = Some(parts[1].parse().unwrap()),
                "FROM_USER_ID" => draft.from_user_id = Some(parts[1].parse().unwrap()),
                "TO_USER_ID" => draft.to_user_id = Some(parts[1].parse().unwrap()),
                "TIMESTAMP" => draft.timestamp = Some(parts[1].parse().unwrap()),
                "AMOUNT" => draft.amount = Some(parts[1].parse().unwrap()),
                "TX_TYPE" => draft.tx_type = Some(TransactionType::from_str(parts[1])?),
                "DESCRIPTION" => draft.description = Some(parts[1].to_string()),
                "STATUS" => draft.status = Some(TransactionStatus::from_str(parts[1])),
                other @ _ => panic!("Unknown field {}", other),
            }
        }
        if !draft.is_empty() {
            let record = Record::from_draft(&draft);
            data.push(record);
        }
        Ok(data)
    }

    pub fn write_to<W: std::io::Write>(writer: &mut W, records: Vec<Record>) -> Result<(), Error> {
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
}

#[cfg(test)]
mod tests {
    use crate::{Record, TransactionType};
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
        let lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();

        assert_eq!(lines.len(), 26);
    }
}
