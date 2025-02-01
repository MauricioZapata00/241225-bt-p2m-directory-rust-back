use std::sync::Arc;
use sqlx::{Error, MySqlPool};
use async_trait::async_trait;
use crate::db::mysql::banks::entity::bank_entity::BankEntity;

#[async_trait]
pub trait BankRepository {
    async fn find_bank_by_bank_code(&self, bank_code: &String)
        -> Result<Option<BankEntity>, Error>;

}

pub struct SqlxBankRepository {
    pool: Arc<MySqlPool>,
}

impl SqlxBankRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankRepository for SqlxBankRepository {
    async fn find_bank_by_bank_code(&self, bank_code: &String) -> Result<Option<BankEntity>, Error> {
        sqlx::query_as::<_, BankEntity>(
            "SELECT * FROM banks WHERE bank_code = ?
                AND status_id = 1"
        )
            .bind(bank_code)
            .fetch_optional(&*self.pool)
            .await
    }
}