use domain::exception::database_error::DatabaseError;
use domain::model::commerce::Commerce;

pub trait CommerceRepositoryPort {
    async fn create_commerce(&self, commerce: &Commerce) -> Result<Commerce, DatabaseError>;
    async fn commerce_does_not_exist_by_ruc_and_alias(&self, ruc: &String, alias: &String)
        -> Result<bool, DatabaseError>;
    async fn commerce_exists_by_ruc_or_legal_business_name(&self, ruc: &String,
                                                           legal_business_name: &String)
        -> Result<bool, DatabaseError>;
}