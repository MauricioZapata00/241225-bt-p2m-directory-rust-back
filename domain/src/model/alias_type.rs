use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasType {
    pub id: i64,  
    pub description: String,
}

impl AliasType {
    pub fn new(id: i64, description: String) -> Self {
        Self {
            id,
            description,
        }
    }
}