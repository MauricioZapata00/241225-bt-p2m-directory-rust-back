use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{AnyPool, Error};
use crate::db::mssql::banks::repository::bank_repository::{BankRepository, SqlxBankRepository};
use crate::db::mssql::commerces::entity::account_entity::AccountEntity;
use crate::db::mssql::commerces::entity::commerce_entity::CommerceEntity;
use crate::db::mssql::commerces::entity::wrappers::commerce_db_info_wrapper::CommerceDbInfoWrapper;
use crate::db::mssql::commerces::repository::account_repository::{AccountRepository, SqlxAccountRepository};
use crate::db::mssql::commerces::repository::commerce_status_repository::{CommerceStatusRepository, SqlxCommerceStatusRepository};

#[async_trait]
pub trait CommerceRepository {
    async fn find_commerce_by_ruc_or_alias(&self, ruc: &String, alias_value: &String)
        -> Result<Option<CommerceEntity>, Error>;
    async fn find_commerce_by_ruc_or_legal_business_name(&self, ruc: &String,
                                                         legal_business_name: &String)
        -> Result<Option<CommerceEntity>, Error>;
    async fn create_commerce(&self, commerce_entity: &CommerceEntity, bank_code: &String,
                             account_number: &String)
        -> Result<Option<CommerceDbInfoWrapper>, Error>;

}

pub struct SqlxCommerceRepository {
    pool: Arc<AnyPool>,
    sqlx_account_repository: SqlxAccountRepository,
    sqlx_bank_repository: SqlxBankRepository,
    sqlx_commerce_status_repository: SqlxCommerceStatusRepository
}

impl SqlxCommerceRepository {
    pub fn new(pool: Arc<AnyPool>,
               sqlx_account_repository: SqlxAccountRepository,
               sqlx_bank_repository: SqlxBankRepository,
               sqlx_commerce_status_repository: SqlxCommerceStatusRepository) -> Self {
        Self { pool,
            sqlx_account_repository,
            sqlx_bank_repository,
            sqlx_commerce_status_repository
        }
    }

    async fn validate_account_entity_inserted_and_insert_commerce(&self, commerce_entity: &CommerceEntity,
                                                                  account_entity: &Option<AccountEntity>)
        -> Result<Option<CommerceDbInfoWrapper>, Error> {
        match account_entity {
            None => {Err(Error::Database(Box::from("Account could not be inserted")))}
            Some(account_inserted) => {
                let commerce_entity_stored = sqlx::query_as::<_, CommerceEntity>(
                    "INSERT INTO dbo.commerces (alias, alias_type_id, legal_business_name, account_id,
                              ruc, commerce_status_id)
            OUTPUT INSERTED.*
             VALUES (@p1, @p2, @p3, @p4, @p5, 1);"
                )
                    .bind(&commerce_entity.alias)
                    .bind(&commerce_entity.alias_type_id)
                    .bind(&commerce_entity.legal_business_name)
                    .bind(&account_inserted.account_id)
                    .bind(&commerce_entity.ruc)
                    .fetch_optional(&self.pool)
                    .await?.unwrap();
                let commerce_status_entity = self
                    .sqlx_commerce_status_repository
                    .find_commerce_status_by_id(&1).await?;
                Ok(Option::from(CommerceDbInfoWrapper {
                    id_commerce: commerce_entity_stored.id_commerce,
                    alias: commerce_entity_stored.alias.clone(),
                    alias_type_id: commerce_entity_stored.alias_type_id,
                    legal_business_name: commerce_entity_stored.legal_business_name.clone(),
                    account_id: account_inserted.account_id,
                    account_number: account_inserted.account_number.clone(),
                    bank_code: account_inserted.bank_code.clone(),
                    bank_id: account_inserted.bank_id,
                    ruc: commerce_entity_stored.ruc.clone(),
                    commerce_status_id: commerce_entity_stored.commerce_status_id,
                    commerce_status_name: commerce_status_entity.unwrap().status_name.clone()
                }))
            }
        }
    }
}

impl CommerceRepository for SqlxCommerceRepository {
    async fn find_commerce_by_ruc_or_alias(&self, ruc: &String, alias_value: &String)
        -> Result<Option<CommerceEntity>, Error> {
        let alias_with_at_sign = format!("@{}", alias_value);
        sqlx::query_as::<_, CommerceEntity>(
            "SELECT * FROM dbo.commerces WHERE ruc = @p1
                OR alias = @p2 AND commerce_status_id = 1"
        )
            .bind(ruc)
            .bind(&alias_with_at_sign)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_commerce_by_ruc_or_legal_business_name(&self, ruc: &String,
                                                         legal_business_name: &String)
        -> Result<Option<CommerceEntity>, Error> {
        sqlx::query_as::<_, CommerceEntity>(
            "SELECT * FROM dbo.commerces WHERE ruc = @p1
                OR legal_business_name = @p2 AND commerce_status_id = 1"
        )
            .bind(ruc)
            .bind(legal_business_name)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create_commerce(&self, commerce_entity: &CommerceEntity, bank_code: &String,
                             account_number: &String)
        -> Result<Option<CommerceDbInfoWrapper>, Error>
    {
        let bank_entity = self.sqlx_bank_repository.find_bank_by_bank_code(bank_code)
            .await?;

        match bank_entity {
            Some(bank_entity) => {
                let account_entity = self.sqlx_account_repository
                    .insert_new_account(account_number, bank_code, &bank_entity.bank_id).await?;
                self.validate_account_entity_inserted_and_insert_commerce(commerce_entity,
                                                                          &account_entity)
            },
            None => Err(Error::ColumnNotFound(format!("Bank not found with bank_code: {}", bank_code)))
        }


    }


}

