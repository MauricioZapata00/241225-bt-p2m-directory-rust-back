use std::error::Error;
use domain::exception::commerce_error::CommerceError;
use domain::model::commerce::Commerce;
use crate::use_case::commerces::validate_commerce_to_store::ValidateCommerceToStore;

pub struct ValidateCommerceToStoreService {}

impl ValidateCommerceToStore for ValidateCommerceToStoreService {
    fn process(&self, commerce: Commerce) -> Result<Commerce, dyn Error> {

    }
}

fn validate_null_elements(commerce: &Commerce) -> Result<(), CommerceError> {
    if commerce.commerce_id <= 0 {
        return Err(CommerceError::not_valid_alias_type());
    }
    if commerce.alias.trim().is_empty() {
        return Err(CommerceError::not_valid_alias_format());
    }
    if commerce.legal_business_name.trim().is_empty() {
        return Err(CommerceError::not_valid_legal_business());
    }
    if commerce.ruc.trim().is_empty() {
        return Err(CommerceError::not_valid_ruc());
    }
    Ok(())
}

