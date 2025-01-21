use domain::exception::database_error::DatabaseError;

pub trait BankRepositoryPort {
    async fn validate_if_bank_exists_exists_by_bank_code(&self, bank_code: &String)
                                                         -> Result<bool, DatabaseError>;
}