use std::error::Error;
use std::fmt;
use tracing::error;
use crate::exception::error_attributes::ErrorAttributes;

#[derive(Debug)]
pub enum UserError {
    DebtorNull(ErrorAttributes),
    DebtorNotFound(ErrorAttributes),
    CreditorNotFound(ErrorAttributes),
    DebtorInvalidAliasType(ErrorAttributes),
    DebtorInvalidAlias(ErrorAttributes),
    DebtorInvalidCountryCode(ErrorAttributes),
    BankCodeNullOrEmptyForDebtor(ErrorAttributes),
    BankCodeErrorValue(ErrorAttributes),
    DebtorInactive(ErrorAttributes),
    CreditorNull(ErrorAttributes),
    CreditorInvalidAliasType(ErrorAttributes),
    CreditorInvalidAlias(ErrorAttributes),
    CreditorInvalidCountryCode(ErrorAttributes),
}

impl UserError {
    pub fn debtor_null() -> Self {
        error!("Debtor user is null");
        UserError::DebtorNull(ErrorAttributes::new(
            String::from("ERR-076"),
            String::from("Debitor nulo o vacio"),
        ))
    }

    pub fn debtor_not_found() -> Self {
        error!("Debtor user is not found");
        UserError::DebtorNotFound(ErrorAttributes::new(
            String::from("ERR-034"),
            String::from("Debitor no encontrado"),
        ))
    }

    pub fn creditor_not_found() -> Self {
        error!("Creditor user is not found");
        UserError::CreditorNotFound(ErrorAttributes::new(
            String::from("ERR-035"),
            String::from("Creditor no encontrado"),
        ))
    }

    pub fn debtor_invalid_alias_type() -> Self {
        error!("Debtor user has not valid alias type");
        UserError::DebtorInvalidAliasType(ErrorAttributes::new(
            String::from("ERR-077"),
            String::from("Tipo de alias invalido para debitor"),
        ))
    }

    pub fn debtor_invalid_alias() -> Self {
        error!("Debtor user has invalid alias");
        UserError::DebtorInvalidAlias(ErrorAttributes::new(
            String::from("ERR-078"),
            String::from("Formato de alias invalido para debitor"),
        ))
    }

    pub fn debtor_invalid_country_code() -> Self {
        error!("Debtor user has invalid country code");
        UserError::DebtorInvalidCountryCode(ErrorAttributes::new(
            String::from("ERR-079"),
            String::from("Codigo de pais invalido para debitor"),
        ))
    }

    pub fn bank_code_null_or_empty_for_debtor() -> Self {
        error!("Bank code null or empty for debtor");
        UserError::BankCodeNullOrEmptyForDebtor(ErrorAttributes::new(
            String::from("ERR-080"),
            String::from("El código del banco es nulo o vacio para el debitor"),
        ))
    }

    pub fn bank_code_error_value() -> Self {
        error!("Invalid bank code format");
        UserError::BankCodeErrorValue(ErrorAttributes::new(
            String::from("ERR-087"),
            String::from("Formato de codigo de banco invalido"),
        ))
    }

    pub fn debtor_inactive() -> Self {
        error!("Debtor user is inactive");
        UserError::DebtorInactive(ErrorAttributes::new(
            String::from("ERR-094"),
            String::from("Alias debitor inactivo"),
        ))
    }

    pub fn creditor_null() -> Self {
        error!("Creditor user is null");
        UserError::CreditorNull(ErrorAttributes::new(
            String::from("ERR-081"),
            String::from("Creditor nulo o vacio"),
        ))
    }

    pub fn creditor_invalid_alias_type() -> Self {
        error!("Creditor user has invalid alias type");
        UserError::CreditorInvalidAliasType(ErrorAttributes::new(
            String::from("ERR-082"),
            String::from("Tipo de alias invalido para creditor"),
        ))
    }

    pub fn creditor_invalid_alias() -> Self {
        error!("Creditor user has invalid alias");
        UserError::CreditorInvalidAlias(ErrorAttributes::new(
            String::from("ERR-083"),
            String::from("Formato de alias invalido para creditor"),
        ))
    }

    pub fn creditor_invalid_country_code() -> Self {
        error!("Creditor user has invalid country code");
        UserError::CreditorInvalidCountryCode(ErrorAttributes::new(
            String::from("ERR-084"),
            String::from("Codigo de pais invalido para creditor"),
        ))
    }

    pub fn get_code(&self) -> &str {
        match self {
            UserError::DebtorNull(attrs) => attrs.get_code(),
            UserError::DebtorNotFound(attrs) => attrs.get_code(),
            UserError::CreditorNotFound(attrs) => attrs.get_code(),
            UserError::DebtorInvalidAliasType(attrs) => attrs.get_code(),
            UserError::DebtorInvalidAlias(attrs) => attrs.get_code(),
            UserError::DebtorInvalidCountryCode(attrs) => attrs.get_code(),
            UserError::BankCodeNullOrEmptyForDebtor(attrs) => attrs.get_code(),
            UserError::BankCodeErrorValue(attrs) => attrs.get_code(),
            UserError::DebtorInactive(attrs) => attrs.get_code(),
            UserError::CreditorNull(attrs) => attrs.get_code(),
            UserError::CreditorInvalidAliasType(attrs) => attrs.get_code(),
            UserError::CreditorInvalidAlias(attrs) => attrs.get_code(),
            UserError::CreditorInvalidCountryCode(attrs) => attrs.get_code(),
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            UserError::DebtorNull(attrs) => attrs.get_message(),
            UserError::DebtorNotFound(attrs) => attrs.get_message(),
            UserError::CreditorNotFound(attrs) => attrs.get_message(),
            UserError::DebtorInvalidAliasType(attrs) => attrs.get_message(),
            UserError::DebtorInvalidAlias(attrs) => attrs.get_message(),
            UserError::DebtorInvalidCountryCode(attrs) => attrs.get_message(),
            UserError::BankCodeNullOrEmptyForDebtor(attrs) => attrs.get_message(),
            UserError::BankCodeErrorValue(attrs) => attrs.get_message(),
            UserError::DebtorInactive(attrs) => attrs.get_message(),
            UserError::CreditorNull(attrs) => attrs.get_message(),
            UserError::CreditorInvalidAliasType(attrs) => attrs.get_message(),
            UserError::CreditorInvalidAlias(attrs) => attrs.get_message(),
            UserError::CreditorInvalidCountryCode(attrs) => attrs.get_message(),
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in user with error code: {} \tError is: {}", self.get_code(), self.get_message())
    }
}

impl Error for UserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // This error doesn't wrap another error, so return None
        None
    }

    fn description(&self) -> &str {
        self.get_message()
    }
}