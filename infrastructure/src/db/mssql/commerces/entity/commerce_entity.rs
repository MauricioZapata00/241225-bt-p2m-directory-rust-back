use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CommerceEntity {
    #[sqlx(rename = "id_commerce")]
    pub id_commerce: i64,
    #[sqlx(rename = "alias")]
    pub alias: String,
    #[sqlx(rename = "alias_type_id")]
    pub alias_type_id: i64,
    #[sqlx(rename = "legal_business_name")]
    pub legal_business_name: String,
    #[sqlx(rename = "account_id")]
    pub account_id: i64,
    #[sqlx(rename = "ruc")]
    pub ruc: String,
    #[sqlx(rename = "commerce_status_id")]
    pub commerce_status_id: i64,
}