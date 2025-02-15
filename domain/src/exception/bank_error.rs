use crate::exception::error_attributes::ErrorAttributes;
use std::fmt;
use std::error::Error;
use tracing::error;

#[derive(Debug)]
pub enum BankError {
    CreditorBankNotActive(ErrorAttributes),
    CreditorBankNotFound(ErrorAttributes),
    DebtorBankNotActive(ErrorAttributes),
}

impl BankError {
    pub fn creditor_bank_not_active() -> Self {
        error!("Creditor bank is currently not active");
        BankError::CreditorBankNotActive(ErrorAttributes::new(
            String::from("ERR-088"),
            String::from("Banco de alias commerce no se encuentra activo"),
        ))
    }

    pub fn creditor_bank_not_found() -> Self {
        error!("Creditor bank is not found");
        BankError::CreditorBankNotFound(ErrorAttributes::new(
            String::from("ERR-002"),
            String::from("Banco no encontrado"),
        ))
    }

    pub fn debtor_bank_not_active() -> Self {
        error!("Debtor bank is not active");
        BankError::DebtorBankNotActive(ErrorAttributes::new(
            String::from("ERR-024"),
            String::from("Banco de alias debitor no se encuentra activo"),
        ))
    }

    pub fn get_code(&self) -> &str {
        match self {
            BankError::CreditorBankNotActive(attrs) => attrs.get_code(),
            BankError::CreditorBankNotFound(attrs) => attrs.get_code(),
            BankError::DebtorBankNotActive(attrs) => attrs.get_code(),
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            BankError::CreditorBankNotActive(attrs) => attrs.get_message(),
            BankError::CreditorBankNotFound(attrs) => attrs.get_message(),
            BankError::DebtorBankNotActive(attrs) => attrs.get_message(),
        }
    }
}

impl fmt::Display for BankError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in bank with error code: {} \tError is: {}", self.get_code(), self.get_message())
    }
}

impl Error for BankError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // This error doesn't wrap another error, so return None
        None
    }

    fn description(&self) -> &str {
        self.get_message()
    }
}

unsafe impl Send for BankError {}
unsafe impl Sync for BankError {}