use crate::exception::error_attributes::ErrorAttributes;

#[derive(Debug)]
pub enum BankError {
    CreditorBankNotActive(ErrorAttributes),
    CreditorBankNotFound(ErrorAttributes),
    DebtorBankNotActive(ErrorAttributes),
}

impl BankError {
    pub fn creditor_bank_not_active() -> Self {
        BankError::CreditorBankNotActive(ErrorAttributes::new(
            String::from("ERR-088"),
            String::from("Banco de alias commerce no se encuentra activo"),
        ))
    }

    pub fn creditor_bank_not_found() -> Self {
        BankError::CreditorBankNotFound(ErrorAttributes::new(
            String::from("ERR-002"),
            String::from("Banco no encontrado"),
        ))
    }

    pub fn debtor_bank_not_active() -> Self {
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