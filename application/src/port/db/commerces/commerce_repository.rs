use domain::exception::database_error::DatabaseError;
use domain::model::commerce::Commerce;

pub trait CommerceRepository {
    async fn create_commerce(commerce: &Commerce) -> Result<Commerce, DatabaseError>;
    async fn validate_if_commerce_does_not_exists_by_ruc_and_alias(ruc: &String, alias: &String)
        -> Result<bool, DatabaseError>;
}