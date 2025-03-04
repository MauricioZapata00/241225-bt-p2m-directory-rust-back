use crate::exception::error_attributes::ErrorAttributes;
use std::fmt;
use std::error::Error;
use tracing::error;

#[derive(Debug)]
pub enum CommerceError {
    NotValidAliasType(ErrorAttributes),
    NotValidAliasFormat(ErrorAttributes),
    NotValidLegalBusiness(ErrorAttributes),
    NotValidRuc(ErrorAttributes),
    NotValidAccountFormat(ErrorAttributes),
    AliasAlreadyExists(ErrorAttributes),
    BankNotFound(ErrorAttributes),
    RucLegalBusinessDoesNotMatch(ErrorAttributes),
    CommerceBankAccountIsEmptyOrNull(ErrorAttributes),
    BankCodeIsEmptyOrNull(ErrorAttributes),
    NotValidFormatBank(ErrorAttributes),
    CommerceInactive(ErrorAttributes),
}

impl CommerceError {

    pub fn not_valid_alias_type() -> Self {
        error!("Not valid alias type");
        CommerceError::NotValidAliasType(ErrorAttributes::new(
            String::from("ERR-009"),
            String::from("Tipo de alias no valido"),
        ))
    }

    pub fn not_valid_alias_format() -> Self {
        error!("Not valid alias format");
        CommerceError::NotValidAliasFormat(ErrorAttributes::new(
            String::from("ERR-003"),
            String::from("Formato de alias no valido"),
        ))
    }

    pub fn not_valid_legal_business() -> Self {
        error!("Not valid legal business name");
        CommerceError::NotValidLegalBusiness(ErrorAttributes::new(
            String::from("ERR-090"),
            String::from("Formato de razon social no valida"),
        ))
    }

    pub fn not_valid_ruc() -> Self {
        error!("Not valid ruc");
        CommerceError::NotValidRuc(ErrorAttributes::new(
            String::from("ERR-091"),
            String::from("Formato de RUC no valido"),
        ))
    }

    pub fn not_valid_account_format() -> Self {
        error!("Not valid account format");
        CommerceError::NotValidAccountFormat(ErrorAttributes::new(
            String::from("ERR-005"),
            String::from("Formato de cuenta no valido"),
        ))
    }

    pub fn alias_already_exists() -> Self {
        error!("Alias already exists");
        CommerceError::AliasAlreadyExists(ErrorAttributes::new(
            String::from("ERR-008"),
            String::from("El Alias ya se encuentra registrado"),
        ))
    }

    pub fn bank_not_found() -> Self {
        error!("Bank not found");
        CommerceError::BankNotFound(ErrorAttributes::new(
            String::from("ERR-002"),
            String::from("Banco no encontrado"),
        ))
    }

    pub fn ruc_legal_business_does_not_match() -> Self {
        error!("Ruc and legal business does not match");
        CommerceError::RucLegalBusinessDoesNotMatch(ErrorAttributes::new(
            String::from("ERR-093"),
            String::from("El ruc y la razon social no coinciden"),
        ))
    }

    pub fn commerce_bank_account_is_empty_or_null() -> Self {
        error!("Commerce Bank account is empty or null");
        CommerceError::CommerceBankAccountIsEmptyOrNull(ErrorAttributes::new(
            String::from("ERR-001"),
            String::from("Vacio o nulo para campos obligatorios"),
        ))
    }

    pub fn bank_code_is_empty_or_null() -> Self {
        error!("Bank code is empty or null");
        CommerceError::BankCodeIsEmptyOrNull(ErrorAttributes::new(
            String::from("ERR-072"),
            String::from("El codigo del banco no puede ser nulo o vacio"),
        ))
    }

    pub fn not_valid_format_bank() -> Self {
        error!("Not valid format bank");
        CommerceError::NotValidFormatBank(ErrorAttributes::new(
            String::from("ERR-087"),
            String::from("Formato de codigo de banco invalido"),
        ))
    }

    pub fn commerce_inactive() -> Self {
        error!("Commerce is currently inactive");
        CommerceError::CommerceInactive(ErrorAttributes::new(
            String::from("ERR-095"),
            String::from("Creditor inactivo"),
        ))
    }

    pub fn get_code(&self) -> &str {
        match self {
            CommerceError::NotValidAliasType(attrs) => attrs.get_code(),
            CommerceError::NotValidAliasFormat(attrs) => attrs.get_code(),
            CommerceError::NotValidLegalBusiness(attrs) => attrs.get_code(),
            CommerceError::NotValidRuc(attrs) => attrs.get_code(),
            CommerceError::NotValidAccountFormat(attrs) => attrs.get_code(),
            CommerceError::AliasAlreadyExists(attrs) => attrs.get_code(),
            CommerceError::BankNotFound(attrs) => attrs.get_code(),
            CommerceError::RucLegalBusinessDoesNotMatch(attrs) => attrs.get_code(),
            CommerceError::CommerceBankAccountIsEmptyOrNull(attrs) => attrs.get_code(),
            CommerceError::BankCodeIsEmptyOrNull(attrs) => attrs.get_code(),
            CommerceError::NotValidFormatBank(attrs) => attrs.get_code(),
            CommerceError::CommerceInactive(attrs) => attrs.get_code(),
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            CommerceError::NotValidAliasType(attrs) => attrs.get_message(),
            CommerceError::NotValidAliasFormat(attrs) => attrs.get_message(),
            CommerceError::NotValidLegalBusiness(attrs) => attrs.get_message(),
            CommerceError::NotValidRuc(attrs) => attrs.get_message(),
            CommerceError::NotValidAccountFormat(attrs) => attrs.get_message(),
            CommerceError::AliasAlreadyExists(attrs) => attrs.get_message(),
            CommerceError::BankNotFound(attrs) => attrs.get_message(),
            CommerceError::RucLegalBusinessDoesNotMatch(attrs) => attrs.get_message(),
            CommerceError::CommerceBankAccountIsEmptyOrNull(attrs) => attrs.get_message(),
            CommerceError::BankCodeIsEmptyOrNull(attrs) => attrs.get_message(),
            CommerceError::NotValidFormatBank(attrs) => attrs.get_message(),
            CommerceError::CommerceInactive(attrs) => attrs.get_message(),
        }
    }
}

impl fmt::Display for CommerceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in commerce with error code: {} \tError is: {}", self.get_code(), self.get_message())
    }
}

impl Error for CommerceError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // This error doesn't wrap another error, so return None
        None
    }

    fn description(&self) -> &str {
        self.get_message()
    }
}

unsafe impl Send for CommerceError {}
unsafe impl Sync for CommerceError {}