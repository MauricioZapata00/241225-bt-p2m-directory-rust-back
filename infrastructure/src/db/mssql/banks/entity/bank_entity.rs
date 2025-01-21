use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct BankEntity {
    #[sqlx(rename = "bank_id")]
    pub bank_id: i64,
    #[sqlx(rename = "bank_name")]
    pub bank_name: String,
    #[sqlx(rename = "bank_code")]
    pub bank_code: String,
    #[sqlx(rename = "contact_name")]
    pub contact_name: String,
    #[sqlx(rename = "contact_mail")]
    pub contact_mail: String,
    #[sqlx(rename = "notification_mail")]
    pub notification_mail: String,
    #[sqlx(rename = "contact_phone")]
    pub contact_phone: String,
    #[sqlx(rename = "bank_ruc")]
    pub bank_ruc: String,
    #[sqlx(rename = "status_id")]
    pub status_id: i64,
}