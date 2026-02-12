pub mod csv_format;
pub mod text_format;

#[derive(Debug)]
pub struct Record {
    pub tx_id: u64,
    tx_type: String,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    timestamp: i64,
    status: String,
    description: String,
}
