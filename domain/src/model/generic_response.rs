use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericResponse {
    pub message_code: String,
    pub message_type: String,
    pub message: String
}

impl GenericResponse {
    pub fn new(message_code: String, message_type: String, message: String) -> Self {
        Self {
            message_code,
            message_type,
            message,
        }
    }
}