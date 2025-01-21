use application::port::db::commerces::commerce_repository_port::CommerceRepositoryPort;
use domain::exception::database_error::DatabaseError;
use domain::model::commerce::Commerce;
use crate::db::mssql::commerces::entity::commerce_entity::CommerceEntity;
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
        match self.commerce_repository.create_commerce() {  }
    }

    async fn commerce_does_not_exist_by_ruc_and_alias(&self, ruc: &String, 
                                                      alias: &String) 
        -> Result<bool, DatabaseError> {
        
    }

    async fn commerce_exists_by_ruc_or_legal_business_name(&self, ruc: &String, 
                                                           legal_business_name: &String) 
        -> Result<bool, DatabaseError> {
        
    }
}

fn map_commerce_to_entity(commerce: &Commerce) -> CommerceEntity {
    CommerceEntity {
        id_commerce: commerce.commerce_id,
        alias: commerce.alias.clone(),
        alias_type_id: commerce.alias_type,
        legal_business_name: commerce.legal_business_name.clone(),
        account_id: commerce.account.account_id, // Assuming Account has account_id field
        ruc: commerce.ruc,
        commerce_status_id: commerce.commerce_status.commerce_status_id, // Assuming CommerceStatus has commerce_status_id field
    }
}