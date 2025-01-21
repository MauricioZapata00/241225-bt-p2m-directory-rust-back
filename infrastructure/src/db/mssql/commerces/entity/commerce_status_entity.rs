use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct CommerceStatusEntity {
    #[sqlx(rename = "commerce_status_id")]
    pub commerce_status_id: i64,
    #[sqlx(rename = "status_name")]
    pub status_name: String,
}