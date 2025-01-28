use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankStatus {
    pub status_name: String
}
impl BankStatus {
    pub fn new(status_name: String) -> Self {
        Self {
            status_name,
        }
    }
}