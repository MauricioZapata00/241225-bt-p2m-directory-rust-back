use std::error::Error;
use domain::model::commerce::Commerce;

pub trait ValidateCommerceToStore {
    fn process(&self, commerce: Commerce) -> Result<Commerce, dyn Error>;
}