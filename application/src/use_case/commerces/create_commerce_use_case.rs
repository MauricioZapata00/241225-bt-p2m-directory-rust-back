use std::error::{Error as StdError};
use domain::model::commerce::Commerce;

pub trait CreateCommerceUseCase {
    async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn StdError + Send + Sync>>;
}