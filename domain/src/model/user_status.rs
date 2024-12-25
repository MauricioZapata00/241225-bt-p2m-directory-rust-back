use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStatus {
    pub status_id: i64,
    pub status_name: String,
}

impl UserStatus {
    pub fn new(status_id: i64, status_name: String) -> Self {
        Self {
            status_id,
            status_name,
        }
    }
}