use std::error::Error;
use domain::model::commerce::Commerce;

pub trait ValidateCommerceToStore {
    async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn Error>>;
}