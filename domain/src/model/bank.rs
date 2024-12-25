use serde::{Serialize, Deserialize};
use crate::model::bank_status::BankStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub bank_name: String,
    pub bank_code: String,
    pub contact_name: String,
    pub contact_mail: String,
    pub notification_mail: String,
    pub contact_phone: String,
    pub bank_ruc: String,
    pub bank_status: BankStatus,
    pub ocp_cert_mtls: String,
}
impl Bank {
    pub fn new(
        bank_name: String,
        bank_code: String,
        contact_name: String,
        contact_mail: String,
        notification_mail: String,
        contact_phone: String,
        bank_ruc: String,
        bank_status: BankStatus,
        ocp_cert_mtls: String,
    ) -> Self {
        Self {
            bank_name,
            bank_code,
            contact_name,
            contact_mail,
            notification_mail,
            contact_phone,
            bank_ruc,
            bank_status,
            ocp_cert_mtls,
        }
    }
}