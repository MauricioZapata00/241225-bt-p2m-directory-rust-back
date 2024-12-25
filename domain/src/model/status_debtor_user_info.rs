use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusDebtorUserInfo {
    pub alias_value: String,
    pub bank_code: String,
    pub country_code: i32,
}

impl StatusDebtorUserInfo {
    pub fn new(alias_value: String, bank_code: String, country_code: i32) -> Self {
        Self {
            alias_value,
            bank_code,
            country_code,
        }
    }
}