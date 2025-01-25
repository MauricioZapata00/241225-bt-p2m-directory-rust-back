use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CommerceDbInfoWrapper {
    pub id_commerce: i64,
    pub alias: String,
    pub alias_type_id: i64,
    pub legal_business_name: String,
    pub account_id: i64,
    pub account_number: String,
    pub bank_code: String,
    pub bank_id: i64,
    pub ruc: String,
    pub commerce_status_id: i64,
    pub commerce_status_name: String
}

impl CommerceDbInfoWrapper {
    pub fn new(id_commerce: i64,
               alias: String, alias_type_id: i64,
               legal_business_name: String,
               account_id: i64,
               account_number: String,
               bank_code: String,
               bank_id: i64,
               ruc: String,
               commerce_status_id: i64,
               commerce_status_name: String) -> Self {
        Self { id_commerce,
            alias,
            alias_type_id,
            legal_business_name,
            account_id,
            account_number,
            bank_code,
            bank_id,
            ruc,
            commerce_status_id,
            commerce_status_name
        }
    }
}

