use std::error::Error;
use domain::exception::commerce_error::CommerceError;
use domain::model::commerce::Commerce;

pub trait ValidateCommerceToStore {
    fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn Error>>;
}