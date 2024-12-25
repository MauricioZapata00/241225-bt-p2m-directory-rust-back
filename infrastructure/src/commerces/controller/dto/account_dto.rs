use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountCommand {
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    #[serde(rename = "bankCode")]
    pub bank_code: String,
}

impl AccountCommand {
    pub fn new(account_number: String, bank_code: String) -> Self {
        Self {
            account_number,
            bank_code,
        }
    }
}