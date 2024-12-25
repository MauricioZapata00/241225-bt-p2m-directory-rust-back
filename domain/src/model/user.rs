use serde::{Serialize, Deserialize};
use crate::model::account::Account;
use crate::model::user_status::UserStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub alias_type: i64,
    pub alias: String,
    pub country: i32,
    pub account: Account,
    pub user_status: UserStatus,
}

impl User {
    pub fn new(
        alias_type: i64,
        alias: String,
        country: i32,
        account: Account,
        user_status: UserStatus,
    ) -> Self {
        Self {
            alias_type,
            alias,
            country,
            account,
            user_status,
        }
    }
}