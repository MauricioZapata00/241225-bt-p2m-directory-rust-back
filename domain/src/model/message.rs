use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub language_code: String,
    pub message_code: String,
    pub message_type: String,
    pub message: String,
}

impl Message {
    pub fn new(
        language_code: String,
        message_code: String,
        message_type: String,
        message: String,
    ) -> Self {
        Self {
            language_code,
            message_code,
            message_type,
            message,
        }
    }
}