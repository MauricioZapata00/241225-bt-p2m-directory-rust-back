use domain::exception::commerce_error::CommerceError;
use domain::model::commerce::Commerce;

pub trait CreateCommerceUseCase {
    fn process(&self, commerce: Commerce) -> Result<Commerce, CommerceError>;
}