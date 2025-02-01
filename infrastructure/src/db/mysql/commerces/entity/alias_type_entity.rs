use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct AliasTypeEntity {
    #[sqlx(rename = "alias_type_id")]
    pub alias_type_id: i64,
    #[sqlx(rename = "description")]
    pub description: String,
}