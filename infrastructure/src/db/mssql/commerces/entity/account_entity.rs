use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AccountEntity {
    #[sqlx(rename = "account_id")]
    pub account_id: i64,
    #[sqlx(rename = "account_number")]
    pub account_number: String,
    #[sqlx(rename = "bank_id")]
    pub bank_id: i64,
}