use std::error::Error as StdError;
use std::sync::Arc;
use tracing::{info, instrument};
use domain::model::commerce::Commerce;
use crate::port::db::commerces::commerce_repository_port::CommerceRepositoryPort;
use crate::use_case::commerces::create_commerce_use_case::CreateCommerceUseCase;
use crate::use_case::commerces::validate_commerce_to_store::ValidateCommerceToStore;

pub struct CreateCommerceService<VC: ValidateCommerceToStore, CR: CommerceRepositoryPort> {
    validate_commerce_to_store_use_case: Arc<VC>,
    commerce_repository: Arc<CR>
}

impl<VC: ValidateCommerceToStore, CR: CommerceRepositoryPort> CreateCommerceService<VC, CR> {
    pub fn new(validate_commerce_to_store_use_case: Arc<VC>, commerce_repository: Arc<CR>) -> Self {
        Self {
            validate_commerce_to_store_use_case,
            commerce_repository
        }
    }
    async fn create_commerce(&self, valid_commerce: &Commerce) -> Result<Commerce, Box<dyn StdError + Send + Sync>>{
        match self.commerce_repository.create_commerce(valid_commerce).await {
            Ok(commerce_created) => Ok(commerce_created),
            Err(e) => Err(e.into()),
        }
    }
}

impl<VC: ValidateCommerceToStore, CR: CommerceRepositoryPort> CreateCommerceUseCase
for CreateCommerceService<VC, CR> {
    async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn StdError + Send + Sync>> {

        info!("Validating commerce: {:?}", commerce);
        let valid_commerce = self.validate_commerce_to_store_use_case
            .process(commerce).await?;
        info!("Creating commerce: {:?}", valid_commerce);
        self.create_commerce(&valid_commerce).await
    }
}