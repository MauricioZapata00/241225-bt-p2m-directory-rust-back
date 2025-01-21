use sqlx::{Error, AnyPool};
use async_trait::async_trait;
use crate::db::mssql::banks::entity::bank_entity::BankEntity;

#[async_trait]
pub trait BankRepository {
    async fn find_bank_by_bank_code(&self, bank_code: &String) -> Result<Option<BankEntity>, Error>;

}

pub struct SqlxBankRepository {
    pool: AnyPool,
}

impl SqlxBankRepository {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BankRepository for SqlxBankRepository {
    async fn find_bank_by_bank_code(&self, bank_code: &String) -> Result<Option<BankEntity>, Error> {
        sqlx::query_as::<_, BankEntity>(
            "SELECT * FROM dbo.banks WHERE bank_code = @p1"
        )
            .bind(bank_code)
            .fetch_optional(&self.pool)
            .await
    }
}