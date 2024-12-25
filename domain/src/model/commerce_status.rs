use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceStatus {
    pub status_name: String,
}

impl CommerceStatus {
    pub fn new(status_name: String) -> Self {
        Self {
            status_name,
        }
    }
}