use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub account_number: String,
    pub bank_code: String,
    pub bank_id: i64,
}
impl Account {
    pub fn new(account_number: String, bank_code: String, bank_id: i64) -> Self {
        Self {
            account_number,
            bank_code,
            bank_id,
        }
    }
}