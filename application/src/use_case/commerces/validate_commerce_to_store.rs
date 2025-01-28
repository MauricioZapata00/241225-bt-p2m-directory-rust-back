use std::error::{Error as StdError, Error};
use domain::model::commerce::Commerce;

pub trait ValidateCommerceToStore {
    async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn StdError + Send + Sync>>;
}