use async_trait::async_trait;
use domain::exception::database_error::DatabaseError;

#[async_trait]
pub trait BankRepositoryPort {
    async fn validate_if_bank_exists_exists_by_bank_code(&self, bank_code: &String)
                                                         -> Result<bool, DatabaseError>;
}