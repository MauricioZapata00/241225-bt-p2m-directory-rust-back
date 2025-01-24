use std::fmt::Error;
use async_trait::async_trait;
use sqlx::{AnyPool, Error};
use crate::db::mssql::banks::entity::bank_entity::BankEntity;
use crate::db::mssql::banks::repository::bank_repository::{BankRepository, SqlxBankRepository};
use crate::db::mssql::commerces::entity::account_entity::AccountEntity;
use crate::db::mssql::commerces::entity::commerce_entity::CommerceEntity;
use crate::db::mssql::commerces::repository::account_repository::{AccountRepository, SqlxAccountRepository};

#[async_trait]
pub trait CommerceRepository {
    async fn find_commerce_by_ruc_or_alias(&self, ruc: &String, alias_value: &String)
        -> Result<Option<CommerceEntity>, Error>;
    async fn find_commerce_by_ruc_or_legal_business_name(&self, ruc: &String,
                                                         legal_business_name: &String)
        -> Result<Option<CommerceEntity>, Error>;
    async fn create_commerce(&self, commerce_entity: &CommerceEntity, bank_code: &String,
                             account_number: &String)
        -> Result<Option<CommerceEntity>, Error>;

}

pub struct SqlxCommerceRepository {
    pool: AnyPool,
    sqlx_account_repository: SqlxAccountRepository,
    sqlx_bank_repository: SqlxBankRepository
}

impl SqlxCommerceRepository {
    pub fn new(pool: AnyPool,
               sqlx_account_repository: SqlxAccountRepository,
               sqlx_bank_repository: SqlxBankRepository) -> Self {
        Self { pool,
            sqlx_account_repository,
            sqlx_bank_repository }
    }

    async fn validate_account_entity_inserted_and_insert_commerce(&self, commerce_entity: &CommerceEntity,
                                                                  account_entity: &Option<AccountEntity>)
                                                                  -> Result<Option<CommerceEntity>, Error> {
        match account_entity {
            None => {Err(Error::Database(Error::new("Account was not inserted")))}
            Some(account_inserted) => {
                sqlx::query_as::<_, CommerceEntity>(
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
                    .await
            }
        }
    }
}

impl CommerceRepository for SqlxCommerceRepository {
    async fn find_commerce_by_ruc_or_alias(&self, ruc: &String, alias_value: &String)
        -> Result<Option<CommerceEntity>, Error> {
        sqlx::query_as::<_, CommerceEntity>(
            "SELECT * FROM dbo.commerces WHERE ruc = @p1
                OR alias = @p2"
        )
            .bind(ruc)
            .bind(alias_value)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_commerce_by_ruc_or_legal_business_name(&self, ruc: &String,
                                                         legal_business_name: &String)
        -> Result<Option<CommerceEntity>, Error> {
        sqlx::query_as::<_, CommerceEntity>(
            "SELECT * FROM dbo.commerces WHERE ruc = @p1
                OR legal_business_name = @p2"
        )
            .bind(ruc)
            .bind(legal_business_name)
            .fetch_optional(&self.pool)
            .await
    }

    async fn create_commerce(&self, commerce_entity: &CommerceEntity, bank_code: &String,
                             account_number: &String)
        -> Result<Option<CommerceEntity>, Error> {
        let bank_entity = self.sqlx_bank_repository.find_bank_by_bank_code(bank_code)
            .await?;

        match bank_entity {
            Some(bank_entity) => {
                let account_entity = self.sqlx_account_repository
                    .insert_new_account(account_number, bank_code, &bank_entity.bank_id).await?;
                self.validate_account_entity_inserted_and_insert_commerce(commerce_entity, &account_entity)
            },
            None => Err(Error::ColumnNotFound(format!("Bank not found with bank_code: {}", bank_code)))
        }


    }


}

