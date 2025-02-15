use rocket::serde::{Deserialize as RocketDeserialize, Serialize as RocketSerialize};


#[derive(Debug, Clone, RocketSerialize, RocketDeserialize)]
pub struct AccountDto {
    #[serde(rename = "accountNumber")]
    pub account_number: String,

    #[serde(rename = "bankCode")]
    pub bank_code: String,
}

impl AccountDto {
    pub fn new(account_number: String, bank_code: String) -> Self {
        Self {
            account_number,
            bank_code,
        }
    }
}