use std::error::{Error as StdError};
use async_trait::async_trait;
use domain::model::commerce::Commerce;

#[async_trait]
pub trait CreateCommerceUseCase {
    async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn StdError + Send + Sync>>;
}