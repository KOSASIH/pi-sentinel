// Genesis block configuration
pub struct Genesis {
    pub timestamp: u64,
    pub block_number: u64,
    pub transactions: Vec<Transaction>,
    pub validators: Vec<Validator>,
}
