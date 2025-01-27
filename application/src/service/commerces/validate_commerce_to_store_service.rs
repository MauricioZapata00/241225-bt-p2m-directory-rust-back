use std::error::Error;
use regex::Regex;
use domain::exception::bank_error::BankError;
use domain::exception::commerce_error::CommerceError;
use domain::model::commerce::Commerce;
use crate::port::db::banks::bank_repository_port::BankRepositoryPort;
use crate::port::db::commerces::commerce_repository_port::CommerceRepositoryPort;
use crate::use_case::commerces::validate_commerce_to_store::ValidateCommerceToStore;


const MIN_LENGTH: u8 = 3;
const MAX_LENGTH: u8 = 25;
const ALIAS_PATTERN: String = format!("^[A-Za-z0-9]{{{},{}}}", MIN_LENGTH, MAX_LENGTH);
const ALIAS_REGEX: Regex = Regex::new(&ALIAS_PATTERN).unwrap();
const RUC_PATTERN: String = String::from("^(?=.*\\d)[0-9-]{1,25}$");
const RUC_REGEX: Regex = Regex::new(&RUC_PATTERN).unwrap();
const ACCOUNT_NUMBER_PATTERN: String = String::from("^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$");
const ACCOUNT_NUMBER_REGEX: Regex = Regex::new(&RUC_PATTERN).unwrap();

pub struct ValidateCommerceToStoreService<BR: BankRepositoryPort, CR: CommerceRepositoryPort> {
    bank_repository: BR,
    commerce_repository: CR
}

impl<BR: BankRepositoryPort, CR: CommerceRepositoryPort> ValidateCommerceToStoreService<BR, CR> {
    pub fn new(bank_repository: BR, commerce_repository: CR) -> Self {
        Self {
            bank_repository,
            commerce_repository
        }
    }

    async fn validate_commerce_logic(&self, commerce: &Commerce) -> Result<(), Box<dyn Error>> {
        match self.commerce_repository
            .commerce_does_not_exist_by_ruc_and_alias(&commerce.ruc, &commerce.alias)
            .await
        {
            Ok(true) => return Err(CommerceError::alias_already_exists().into()),
            Ok(false) => (),
            Err(e) => return Err(e.into()),
        }

        match self.bank_repository
            .validate_if_bank_exists_exists_by_bank_code(&commerce.account.bank_code)
            .await
        {
            Ok(false) => return Err(BankError::creditor_bank_not_found().into()),
            Ok(true) => (),
            Err(e) => return Err(e.into()),
        }

        match self.commerce_repository
            .commerce_exists_by_ruc_or_legal_business_name(&commerce.ruc, &commerce.legal_business_name)
            .await
        {
            Ok(false) => return Err(CommerceError::ruc_legal_business_does_not_match().into()),
            Ok(true) => (),
            Err(e) => return Err(e.into()),
        }

        Ok(())
    }
}

impl<BR: BankRepositoryPort, CR: CommerceRepositoryPort> ValidateCommerceToStore
for ValidateCommerceToStoreService<BR, CR> {
    fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn Error>> {
        validate_commerce_field_formats(&commerce)?;
        let mut commerce_validated = commerce;
        commerce_validated.alias = format!("@{}", commerce_validated.alias);
        commerce_validated.legal_business_name = commerce_validated.legal_business_name.trim()
            .to_string();
        self.validate_commerce_logic(&commerce_validated)?;
        Ok(commerce_validated)
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

