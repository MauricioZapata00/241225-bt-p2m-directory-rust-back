use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusCreditorUserInfo {
    pub alias_value: String,
}

impl StatusCreditorUserInfo {
    pub fn new(alias_value: String) -> Self {
        Self {
            alias_value,
        }
    }
}