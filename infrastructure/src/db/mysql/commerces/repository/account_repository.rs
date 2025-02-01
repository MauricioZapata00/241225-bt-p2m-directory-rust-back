use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Error, MySqlPool};
use crate::db::mysql::commerces::entity::account_entity::AccountEntity;

#[async_trait]
pub trait AccountRepository {
    async fn insert_new_account<'a>(&self, account_number: &'a String,
                                bank_code: &'a String,
                                bank_id: &'a i64)
                                -> Result<Option<AccountEntity>, Error>;
}

pub struct SqlxAccountRepository {
    pool: Arc<MySqlPool>,
}

impl SqlxAccountRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AccountRepository for SqlxAccountRepository {
    async fn insert_new_account<'a>(&self, account_number: &'a String,
                                bank_code: &'a String,
                                bank_id: &'a i64)
                                -> Result<Option<AccountEntity>, Error> {
        let mut tx = self.pool.begin().await?;

        sqlx::query(
            "INSERT INTO accounts (account_number, bank_code, bank_id)
                VALUES (?, ?, ?)"
        )
            .bind(account_number)
            .bind(bank_code)
            .bind(bank_id)
            .execute(&mut *tx)
            .await?;

        let result = sqlx::query_as::<_, AccountEntity>(
            "SELECT * FROM accounts WHERE id = LAST_INSERT_ID()"
        )
            .fetch_optional(&mut *tx)
            .await?;

        tx.commit().await?;

        Ok(result)
    }
}