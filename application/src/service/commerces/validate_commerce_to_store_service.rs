use regex::Regex;
use domain::exception::commerce_error::CommerceError;
use domain::model::commerce::Commerce;
use crate::use_case::commerces::validate_commerce_to_store::ValidateCommerceToStore;


const MIN_LENGTH: u8 = 3;
const MAX_LENGTH: u8 = 25;
const ALIAS_PATTERN: String = format!("^[A-Za-z0-9]{{{},{}}}", MIN_LENGTH, MAX_LENGTH);
const ALIAS_REGEX: Regex = Regex::new(&ALIAS_PATTERN).unwrap();
const RUC_PATTERN: String = String::from("^(?=.*\\d)[0-9-]{1,25}$");
const RUC_REGEX: Regex = Regex::new(&RUC_PATTERN).unwrap();
const ACCOUNT_NUMBER_PATTERN: String = String::from("^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$");
const ACCOUNT_NUMBER_REGEX: Regex = Regex::new(&RUC_PATTERN).unwrap();

pub struct ValidateCommerceToStoreService {
    commerce_repository: String
}

impl ValidateCommerceToStore for ValidateCommerceToStoreService {
    fn process(&self, commerce: Commerce) -> Result<Commerce, CommerceError> {
        validate_commerce_field_formats(&commerce)?;
        Ok(commerce)
    }
}

fn validate_commerce_field_formats(commerce: &Commerce) -> Result<(), CommerceError> {
    if commerce.alias_type != 2 {
        return Err(CommerceError::not_valid_alias_type());
    }
    if is_invalid_alias(&commerce.alias) {
        return Err(CommerceError::not_valid_alias_format());
    }
    if is_invalid_legal_business(commerce.legal_business_name.trim()) {
        return Err(CommerceError::not_valid_legal_business());
    }
    if is_invalid_ruc(&commerce.ruc) {
        return Err(CommerceError::not_valid_ruc());
    }
    if is_invalid_account_number(&commerce.account.account_number) {
        return Err(CommerceError::not_valid_account_format())
    }
    Ok(())
}

fn is_invalid_alias(alias: &String) -> bool {
    !ALIAS_REGEX.is_match(alias)
}

fn is_invalid_legal_business(legal_business_name: &str) -> bool {
    legal_business_name.len() > 255 //I am only going to use ASCII characters
}

fn is_invalid_ruc(ruc: &String) -> bool {
    !RUC_REGEX.is_match(ruc)
}

fn is_invalid_account_number(account_number: &String) -> bool {
    !ACCOUNT_NUMBER_REGEX.is_match(account_number)
}

