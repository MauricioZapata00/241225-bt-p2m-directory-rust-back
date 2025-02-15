use rocket::serde::{Deserialize as RocketDeserialize, Serialize as RocketSerialize};
use tracing::info;
use domain::exception::commerce_error::CommerceError;
use domain::model::account::Account;
use domain::model::commerce::Commerce;
use domain::model::commerce_status::CommerceStatus;
use crate::entrypoint::commerces::dto::account_dto::AccountDto;

#[derive(Debug, Clone, RocketSerialize, RocketDeserialize)]
pub struct CommerceDto {
    #[serde(rename = "commerceId")]
    pub commerce_id: Option<i64>,

    #[serde(rename = "aliasValue")]
    pub alias_value: Option<String>,

    #[serde(rename = "aliasType")]
    pub alias_type: Option<i64>,

    #[serde(rename = "legalBusinessName")]
    pub legal_business_name: Option<String>,

    #[serde(rename = "commerceBankAccount")]
    pub commerce_bank_account: Option<AccountDto>,

    #[serde(rename = "ruc")]
    pub ruc: Option<String>,
}

impl CommerceDto {
    pub fn new(
        commerce_id: Option<i64>,
        alias_value: Option<String>,
        alias_type: Option<i64>,
        legal_business_name: Option<String>,
        commerce_bank_account: Option<AccountDto>,
        ruc: Option<String>,
    ) -> Self {
        Self {
            commerce_id,
            alias_value,
            alias_type,
            legal_business_name,
            commerce_bank_account,
            ruc,
        }
    }

    fn validate(&self) -> Result<(), CommerceError> {
        // Validate required fields
        let commerce_alias_type = self.alias_type
            .ok_or_else(|| CommerceError::not_valid_alias_type())?;

        let alias = self.alias_value
            .as_ref()
            .ok_or_else(|| CommerceError::not_valid_alias_format())?;

        let commerce_legal_business_name = self.legal_business_name
            .as_ref()
            .ok_or_else(|| CommerceError::not_valid_alias_format())?;

        let account = self.commerce_bank_account
            .as_ref()
            .ok_or_else(|| CommerceError::not_valid_account_format())?;

        let commerce_ruc = self.ruc
            .as_ref()
            .ok_or_else(|| CommerceError::not_valid_ruc())?;

        // Now validate the values themselves
        info!("Validating commerce alias type: {}", commerce_alias_type);
        validate_long_number(Some(commerce_alias_type))?;
        info!("Validating alias value: {}", alias);
        validate_null_string_value(alias, CommerceError::not_valid_alias_format)?;
        info!("Validating commerce legal business name: {}", alias);
        validate_null_string_value(commerce_legal_business_name, CommerceError::not_valid_legal_business)?;
        info!("Validating account_dto: {}", account);
        validate_account(account)?;
        info!("Validating commerce ruc value: {}", commerce_ruc);
        validate_null_string_value(commerce_ruc, CommerceError::not_valid_ruc)?;


        Ok(())
    }

    // Convert to domain model after validation
    pub fn to_domain(self) -> Result<Commerce, CommerceError> {
        self.validate()?;

        // After validation, we can safely unwrap since we know the values exist
        Ok(Commerce::new(
            self.commerce_id.unwrap_or(0),
            self.alias_value.unwrap(),
            self.alias_type.unwrap(),
            self.legal_business_name.unwrap(),
            account_dto_to_domain(&self.commerce_bank_account.unwrap()),
            self.ruc.unwrap(),
            CommerceStatus::new(String::from("")),
        ))
    }
}

fn validate_long_number(number: Option<i64>) -> Result<(), CommerceError> {
    number.filter(|&id| id > 0)
        .map(|_| ())
        .ok_or_else(|| CommerceError::not_valid_alias_type())
}

fn validate_account(account: &AccountDto) -> Result<(), CommerceError> {
    // Validate account number
    info!("Validating account_number'{}'", account.account_number);
    validate_null_string_value(&account.account_number, CommerceError::not_valid_account_format)?;

    // Validate bank code
    info!("Validating bank_code'{}'", account.bank_code);
    validate_null_string_value(&account.bank_code, CommerceError::bank_code_is_empty_or_null)?;

    Ok(())
}

fn validate_null_string_value<F>(value: &String, error_fn: F) -> Result<(), CommerceError>
where
    F: FnOnce() -> CommerceError
{
    if value.trim().is_empty() {
        Err(error_fn())
    } else {
        Ok(())
    }
}

fn account_dto_to_domain(account: &AccountDto) -> Account {
    Account::new(
        0,
        account.account_number.clone(),
        account.bank_code.clone(),
        0
    )
}