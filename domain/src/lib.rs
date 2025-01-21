
pub mod model {
    pub mod alias_type;
    pub mod account;
    pub mod bank;
    pub mod bank_status;
    pub mod commerce;
    pub mod commerce_status;
    pub mod country;
    pub mod generic_response;
    pub mod message;
    pub mod status_creditor_user_info;
    pub mod status_debtor_user_info;
    pub mod user;
    pub mod user_status;
}

pub mod exception {
    pub mod bank_error;
    pub mod database_error;
    pub mod commerce_error;
    mod error_attributes;
    pub mod user_error;
}