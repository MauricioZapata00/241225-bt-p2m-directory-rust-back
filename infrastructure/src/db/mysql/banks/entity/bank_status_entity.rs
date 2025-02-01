use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BankStatusEntity {
    #[sqlx(rename = "status_id")]
    pub status_id: i64,
    #[sqlx(rename = "status_name")]
    pub status_name: String,
}