use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub country_id: i64,
    pub country_name: String,
    pub country_code: i32,
}

impl Country {
    pub fn new(country_id: i64, country_name: String, country_code: i32) -> Self {
        Self {
            country_id,
            country_name,
            country_code,
        }
    }
}