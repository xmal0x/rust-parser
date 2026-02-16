pub mod bin_parser {
    use crate::error::ParseError;
    use crate::{Record, TransactionStatus, TransactionType};
    use std::io::{BufReader, BufWriter, Error, ErrorKind, Read, Write};

    const MAGIC: [u8; 4] = *b"YPBN";

    pub fn read_from<R: std::io::Read>(r: R) -> Result<Vec<Record>, ParseError> {
        let mut data: Vec<Record> = Vec::new();
        let mut reader = BufReader::new(r);
        let mut header = [0u8; 8];

        loop {
            match reader.read_exact(&mut header) {
                Ok(_) => {}
                Err(e) => {
                    if e.kind() == ErrorKind::UnexpectedEof {
                        break;
                    } else {
                        return Err(ParseError::Io(e));
                    }
                }
            };

            let magic = &header[0..4];

            if magic != MAGIC {
                panic!("NOT MAGIC");
            }

            let record_size = u32::from_be_bytes(header[4..8].try_into().unwrap());

            let mut body = vec![0u8; record_size as usize];

            let _ = reader.read_exact(&mut body);

            let tx_id = u64::from_be_bytes(body[0..8].try_into().unwrap());
            let tx_type =
                TransactionType::from_byte(u8::from_be_bytes(body[8..9].try_into().unwrap()))?;
            let from_user_id = u64::from_be_bytes(body[9..17].try_into().unwrap());
            let to_user_id = u64::from_be_bytes(body[17..25].try_into().unwrap());
            let amount = i64::from_be_bytes(body[25..33].try_into().unwrap());
            let timestamp = u64::from_be_bytes(body[33..41].try_into().unwrap());
            let status =
                TransactionStatus::from_byte(u8::from_be_bytes(body[41..42].try_into().unwrap()));
            let desc_len = u32::from_be_bytes(body[42..46].try_into().unwrap());

            if desc_len != record_size - 46 {
                panic!("Record damaged");
            }

            let description = String::from_utf8(body[46..].try_into().unwrap()).unwrap();

            let record = Record {
                tx_id,
                tx_type,
                from_user_id,
                to_user_id,
                amount,
                timestamp,
                description,
                status,
            };
            data.push(record);
        }

        Ok(data)
    }

    pub fn write_to<W: std::io::Write>(writer: &mut W, records: Vec<Record>) -> Result<(), Error> {
        let mut buffer = BufWriter::new(writer);
        let mut data: Vec<u8> = Vec::new();

        for record in records {
            let tx_id_bytes = record.tx_id.to_be_bytes();
            let from_user_id_bytes = record.from_user_id.to_be_bytes();
            let to_user_id_bytes = record.to_user_id.to_be_bytes();
            let amount_bytes = record.amount.to_be_bytes();
            let timestamp_bytes = record.timestamp.to_be_bytes();
            let description_bytes = record.description.as_bytes();
            let desc_len_bytes = (description_bytes.len() as u32).to_be_bytes();
            let record_size = ((46 + description_bytes.len()) as u32).to_be_bytes();

            let tx_type_bytes = TransactionType::to_byte(&record.tx_type);

            let status_bytes = TransactionStatus::to_byte(&record.status);

            data.extend_from_slice(&MAGIC);
            data.extend_from_slice(&record_size);
            data.extend_from_slice(&tx_id_bytes);
            data.extend_from_slice(&[tx_type_bytes]);
            data.extend_from_slice(&from_user_id_bytes);
            data.extend_from_slice(&to_user_id_bytes);
            data.extend_from_slice(&amount_bytes);
            data.extend_from_slice(&timestamp_bytes);
            data.extend_from_slice(&[status_bytes]);
            data.extend_from_slice(&desc_len_bytes);
            data.extend_from_slice(&description_bytes);
        }

        let _ = buffer.write_all(&data);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Record, TransactionStatus, TransactionType};
    use std::io::Cursor;

    use super::*;

    const BYTES_MOCK: [u8; 213] = [
        89, 80, 66, 78, 0, 0, 0, 63, 0, 3, 141, 126, 164, 198, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        127, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 100, 0, 0, 1, 124, 56, 148,
        250, 96, 1, 0, 0, 0, 17, 34, 82, 101, 99, 111, 114, 100, 32, 110, 117, 109, 98, 101, 114,
        32, 49, 34, 89, 80, 66, 78, 0, 0, 0, 63, 0, 3, 141, 126, 164, 198, 128, 1, 1, 127, 255,
        255, 255, 255, 255, 255, 255, 127, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0,
        200, 0, 0, 1, 124, 56, 149, 228, 192, 2, 0, 0, 0, 17, 34, 82, 101, 99, 111, 114, 100, 32,
        110, 117, 109, 98, 101, 114, 32, 50, 34, 89, 80, 66, 78, 0, 0, 0, 63, 0, 3, 141, 126, 164,
        198, 128, 2, 2, 8, 80, 104, 216, 118, 118, 194, 104, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 44, 0, 0, 1, 124, 56, 150, 207, 32, 0, 0, 0, 0, 17, 34, 82, 101, 99, 111, 114, 100,
        32, 110, 117, 109, 98, 101, 114, 32, 51, 34,
    ];

    fn records_mock() -> [Record; 3] {
        [
            Record {
                tx_id: 1000000000000000,
                tx_type: TransactionType::Deposit,
                from_user_id: 0,
                to_user_id: 9223372036854775807,
                amount: 100,
                timestamp: 1633036860000,
                status: TransactionStatus::Failure,
                description: "\"Record number 1\"".to_string(),
            },
            Record {
                tx_id: 1000000000000001,
                tx_type: TransactionType::Transfer,
                from_user_id: 9223372036854775807,
                to_user_id: 9223372036854775807,
                amount: 200,
                timestamp: 1633036920000,
                status: TransactionStatus::Pending,
                description: "\"Record number 2\"".to_string(),
            },
            Record {
                tx_id: 1000000000000002,
                tx_type: TransactionType::Withdrawal,
                from_user_id: 599094029349995112,
                to_user_id: 0,
                amount: 300,
                timestamp: 1633036980000,
                status: TransactionStatus::Success,
                description: "\"Record number 3\"".to_string(),
            },
        ]
    }

    #[test]
    fn test_load_data() {
        let cursor = Cursor::new(&BYTES_MOCK[..]);
        let r = bin_parser::read_from(cursor).unwrap();

        assert_eq!(r.len(), 3);
        assert_eq!(r[0].timestamp, 1633036860000);
        assert_eq!(r[1].tx_id, 1000000000000001);
        assert_eq!(r[2].from_user_id, 599094029349995112);
    }

    #[test]
    fn test_write_data() {
        let mut cursor = Cursor::new(Vec::new());

        let r = bin_parser::write_to(&mut cursor, records_mock().to_vec());

        assert!(r.is_ok());
        cursor.set_position(0);
        let binary = cursor.into_inner();

        assert_eq!(binary.len(), BYTES_MOCK.len());
    }
}
