use serde::{Serialize, Deserialize};
use crate::model::commerce_status::CommerceStatus;
use crate::model::account::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commerce {
    pub commerce_id: i64,
    pub alias: String,
    pub alias_type: i64,
    pub legal_business_name: String,
    pub account: Account,
    pub ruc: String,
    pub commerce_status: CommerceStatus,
}

impl Commerce {
    pub fn new(
        commerce_id: i64,
        alias: String,
        alias_type: i64,
        legal_business_name: String,
        account: Account,
        ruc: String,
        commerce_status: CommerceStatus,
    ) -> Self {
        Self {
            commerce_id,
            alias,
            alias_type,
            legal_business_name,
            account,
            ruc,
            commerce_status,
        }
    }
}