use application::port::db::commerces::commerce_repository_port::CommerceRepositoryPort;
use domain::exception::database_error::DatabaseError;
use domain::model::account::Account;
use domain::model::commerce::Commerce;
use domain::model::commerce_status::CommerceStatus;
use crate::db::mssql::commerces::entity::commerce_entity::CommerceEntity;
use crate::db::mssql::commerces::entity::wrappers::commerce_db_info_wrapper::CommerceDbInfoWrapper;
use crate::db::mssql::commerces::repository::commerce_repository::{CommerceRepository, SqlxCommerceRepository};

pub struct CommerceRepositoryAdapter {
    commerce_repository: SqlxCommerceRepository
}

impl CommerceRepositoryAdapter {
    pub fn new(commerce_repository: SqlxCommerceRepository) -> Self {
        Self { commerce_repository }
    }
}

impl CommerceRepositoryPort for CommerceRepositoryAdapter {
    async fn create_commerce(&self, commerce: &Commerce) -> Result<Commerce, DatabaseError> {
        let commerce_entity = map_commerce_to_entity(commerce);

        match self.commerce_repository.create_commerce(&commerce_entity, &commerce.account.bank_code,
        &commerce.account.account_number).await {
            Ok(Some(commerce_db_info_wrapper)) => Ok(map_commerce_db_info_wrapper_entity_to_model(&commerce_db_info_wrapper)),
            Ok(None) => Err(DatabaseError::Unexpected(Box::from("Unable to return commerce created"))),
            Err(err) => Err(DatabaseError::Unexpected(err.into()))
        }
    }

    async fn commerce_does_not_exist_by_ruc_and_alias(&self, ruc: &String, 
                                                      alias: &String) 
        -> Result<bool, DatabaseError> {
        match self.commerce_repository.find_commerce_by_ruc_or_alias(ruc, alias).await {
            Ok(Some(_)) => Ok(false),
            Ok(None) => Ok(true),
            Err(err) => Err(DatabaseError::Unexpected(err.into()))
        }
    }

    async fn commerce_exists_by_ruc_or_legal_business_name(&self, ruc: &String, 
                                                           legal_business_name: &String) 
        -> Result<bool, DatabaseError> {
        match self.commerce_repository
            .find_commerce_by_ruc_or_legal_business_name(ruc, legal_business_name).await {
            Ok(Some(commerce_entity)) => {Ok(*commerce_entity.legal_business_name == *legal_business_name
                && *commerce_entity.ruc == *ruc)},
            Ok(None) => Ok(true),
            Err(err) => Err(DatabaseError::Unexpected(err.into()))
        }
    }
}

fn map_commerce_to_entity(commerce: &Commerce) -> CommerceEntity {
    CommerceEntity {
        id_commerce: commerce.commerce_id,
        alias: commerce.alias.clone(),
        alias_type_id: commerce.alias_type,
        legal_business_name: commerce.legal_business_name.clone(),
        account_id: commerce.account.account_id, // Assuming Account has account_id field
        ruc: commerce.ruc.clone(),
        commerce_status_id: 1, // Assuming CommerceStatus has commerce_status_id field
    }
}

fn map_commerce_db_info_wrapper_entity_to_model(commerce_db_info_wrapper: &CommerceDbInfoWrapper)
    -> Commerce {
    Commerce {
        commerce_id: commerce_db_info_wrapper.id_commerce,
        alias: commerce_db_info_wrapper.alias.clone(),
        alias_type: commerce_db_info_wrapper.alias_type_id,
        legal_business_name: commerce_db_info_wrapper.legal_business_name.clone(),
        account: Account {
            account_id: commerce_db_info_wrapper.account_id,
            account_number: commerce_db_info_wrapper.account_number.clone(),
            bank_code: commerce_db_info_wrapper.bank_code.clone(),
            bank_id: commerce_db_info_wrapper.bank_id
        },
        ruc: commerce_db_info_wrapper.ruc.clone(),
        commerce_status: CommerceStatus {
            status_name: commerce_db_info_wrapper.commerce_status_name.clone()
        }
    }
}