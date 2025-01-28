use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{AnyPool, Error};
use crate::db::mssql::commerces::entity::account_entity::AccountEntity;

#[async_trait]
pub trait AccountRepository {
    async fn insert_new_account<'a>(&self, account_number: &'a String,
                                bank_code: &'a String,
                                bank_id: &'a i64)
                                -> Result<Option<AccountEntity>, Error>;
}

pub struct SqlxAccountRepository {
    pool: Arc<AnyPool>,
}

impl SqlxAccountRepository {
    pub fn new(pool: Arc<AnyPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for SqlxAccountRepository {
    async fn insert_new_account<'a>(&self, account_number: &'a String,
                                bank_code: &'a String,
                                bank_id: &'a i64)
                                -> Result<Option<AccountEntity>, Error> {
        sqlx::query_as::<_, AccountEntity>(
            "INSERT INTO dbo.accounts (account_number, bank_code, bank_id)
                OUTPUT INSERTED.*
                VALUES (@p1, @p2, @p3);"
        )
            .bind(account_number)
            .bind(bank_code)
            .bind(bank_id)
            .fetch_optional(&*self.pool)
            .await
    }
}