use serde::{Serialize, Deserialize};
use crate::commerces::controller::dto::account_dto::AccountCommand;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommerceCommand {
    #[serde(rename = "commerceId")]
    pub commerce_id: i64,

    #[serde(rename = "aliasValue")]
    pub alias_value: String,

    #[serde(rename = "aliasType")]
    pub alias_type: i64,

    #[serde(rename = "legalBusinessName")]
    pub legal_business_name: String,

    #[serde(rename = "commerceBankAccount")]
    pub commerce_bank_account: AccountCommand,

    pub ruc: String,
}

impl CommerceCommand {
    pub fn new(
        commerce_id: i64,
        alias_value: String,
        alias_type: i64,
        legal_business_name: String,
        commerce_bank_account: AccountCommand,
        ruc: String,
    ) -> Self {
        Self {
            commerce_id,
            alias_value,
            alias_type,
            legal_business_name,
            commerce_bank_account,
            ruc,
        }
    }
}