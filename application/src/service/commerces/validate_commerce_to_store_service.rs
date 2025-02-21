use std::error::Error as StdError;
use std::sync::Arc;
use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use tracing::{error, info};
use domain::exception::bank_error::BankError;
use domain::exception::commerce_error::CommerceError;
use domain::model::commerce::Commerce;
use crate::port::db::banks::bank_repository_port::BankRepositoryPort;
use crate::port::db::commerces::commerce_repository_port::CommerceRepositoryPort;
use crate::use_case::commerces::validate_commerce_to_store::ValidateCommerceToStore;


const MIN_LENGTH: u8 = 3;
const MAX_LENGTH: u8 = 25;
lazy_static! {
    static ref ALIAS_REGEX: Regex = {
        let pattern = format!("^[A-Za-z0-9]{{{},{}}}", MIN_LENGTH, MAX_LENGTH);
        Regex::new(&pattern).unwrap()
    };

    static ref RUC_REGEX: Regex = {
        Regex::new("^[0-9-]{1,25}$").unwrap()
    };

    static ref ACCOUNT_NUMBER_REGEX: Regex = {
        Regex::new("^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$").unwrap()
    };
}

pub struct ValidateCommerceToStoreService<BR: BankRepositoryPort, CR: CommerceRepositoryPort> {
    bank_repository: Arc<BR>,
    commerce_repository: Arc<CR>
}

impl<BR: BankRepositoryPort, CR: CommerceRepositoryPort> ValidateCommerceToStoreService<BR, CR> {
    pub fn new(bank_repository: Arc<BR>, commerce_repository: Arc<CR>) -> Self {
        Self {
            bank_repository,
            commerce_repository
        }
    }

    async fn validate_commerce_logic(&self, commerce: &Commerce) -> Result<(), Box<dyn StdError + Send + Sync>> {
        match self.commerce_repository
            .commerce_does_not_exist_by_ruc_and_alias(&commerce.ruc, &commerce.alias)
            .await
        {
            Ok(true) => (),
            Ok(false) => {
                error!("Alias '{}' already exists", commerce.alias);
                return Err(CommerceError::alias_already_exists().into())
            },
            Err(e) => {
                error!("There was an error in the database. Error is: {:?}", e);
                return Err(e.into())
            }
        }

        match self.bank_repository
            .validate_if_bank_exists_exists_by_bank_code(&commerce.account.bank_code)
            .await
        {
            Ok(false) => {
                error!("Creditor bank code '{}' does not exist", commerce.account.bank_code);
                return Err(BankError::creditor_bank_not_found().into())
            },
            Ok(true) => (),
            Err(e) => {
                error!("There was an error in the database. Error is: {:?}", e);
                return Err(e.into())
            }
        }

        match self.commerce_repository
            .commerce_exists_by_ruc_or_legal_business_name(&commerce.ruc, &commerce.legal_business_name)
            .await
        {
            Ok(false) => {
                error!("Ruc: {:?} and legal business name: {:?} does not match", commerce.ruc,
                    commerce.legal_business_name);
                return Err(CommerceError::ruc_legal_business_does_not_match().into())
            },
            Ok(true) => (),
            Err(e) => {
                error!("There was an error in the database. Error is: {:?}", e);
                return Err(e.into())
            }
        }

        Ok(())
    }
}

#[async_trait]
impl<BR, CR> ValidateCommerceToStore
for ValidateCommerceToStoreService<BR, CR>
where
    BR: BankRepositoryPort + Send + Sync + 'static,
    CR: CommerceRepositoryPort + Send + Sync + 'static
{
    async fn process(&self, commerce: Commerce) -> Result<Commerce, Box<dyn StdError + Send + Sync>> {
        info!("Validating commerce field formats");
        validate_commerce_field_formats(&commerce)?;
        info!("Commerce field formats are valid for commerce {:?}", commerce);
        info!("Formatting alias value an erasing blank spaces in legal business name");
        let mut commerce_validated = commerce;
        commerce_validated.alias = format!("@{}", commerce_validated.alias);
        commerce_validated.legal_business_name = commerce_validated.legal_business_name.trim()
            .to_string();
        info!("Validating commerce logic for commerce {:?}", commerce_validated);
        self.validate_commerce_logic(&commerce_validated).await?;
        info!("Commerce logic is valid for commerce {:?}", commerce_validated);
        Ok(commerce_validated)
    }
}

fn validate_commerce_field_formats(commerce: &Commerce) -> Result<(), CommerceError> {
    if commerce.alias_type != 2 {
        error!("Invalid alias type: {}", commerce.alias_type);
        return Err(CommerceError::not_valid_alias_type());
    }
    if is_invalid_alias(&commerce.alias) {
        error!("Invalid alias format: {}", commerce.alias);
        return Err(CommerceError::not_valid_alias_format());
    }
    if is_invalid_legal_business(commerce.legal_business_name.trim()) {
        error!("Invalid legal business name: {}", commerce.legal_business_name);
        return Err(CommerceError::not_valid_legal_business());
    }
    if is_invalid_ruc(&commerce.ruc) {
        error!("Invalid RUC: {}", commerce.ruc);
        return Err(CommerceError::not_valid_ruc());
    }
    if is_invalid_account_number(&commerce.account.account_number) {
        error!("Invalid account number: {}", commerce.account.account_number);
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

