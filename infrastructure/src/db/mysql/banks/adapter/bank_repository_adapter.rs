use std::sync::Arc;
use async_trait::async_trait;
use tracing::error;
use application::port::db::banks::bank_repository_port::BankRepositoryPort;
use domain::exception::database_error::DatabaseError;
use crate::db::mysql::banks::repository::bank_repository::{BankRepository, SqlxBankRepository};

pub struct BankRepositoryAdapter {
    bank_repository: Arc<SqlxBankRepository>
}

impl BankRepositoryAdapter {
    pub fn new(bank_repository: Arc<SqlxBankRepository>) -> Self {
        Self { bank_repository }
    }
}

#[async_trait]
impl BankRepositoryPort for BankRepositoryAdapter {

    async fn validate_if_bank_exists_exists_by_bank_code(&self, bank_code: &String)
        -> Result<bool, DatabaseError> {
        match self.bank_repository.find_bank_by_bank_code(bank_code).await {
            Ok(Some(bank_entity)) => Ok(bank_entity.bank_code == *bank_code),
            Ok(None) => Ok(false),
            Err(err) => {
                error!("There was an error validating if bank exists by bank_code. Error is: {:?}", err);
                Err(DatabaseError::Unexpected(err.into()))
            }
        }
    }
}