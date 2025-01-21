use async_trait::async_trait;
use sqlx::{AnyPool, Error, Row};
use crate::db::mssql::commerces::entity::commerce_entity::CommerceEntity;

#[async_trait]
pub trait CommerceRepository {
    async fn find_commerce_by_ruc_or_alias(&self, ruc: &String, alias_value: &String)
        -> Result<Option<CommerceEntity>, Error>;
    async fn find_commerce_by_ruc_or_legal_business_name(&self, ruc: &String,
                                                         legal_business_name: &String)
        -> Result<Option<CommerceEntity>, Error>;
    async fn create_commerce(&self, commerce_entity: &CommerceEntity)
        -> Result<Option<CommerceEntity>, Error>;

}

pub struct SqlxCommerceRepository {
    pool: AnyPool,
}

impl SqlxCommerceRepository {
    pub fn new(pool: AnyPool) -> Self {
        Self { pool }
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

    async fn create_commerce(&self, commerce_entity: &CommerceEntity)
        -> Result<Option<CommerceEntity>, Error> {
        sqlx::query_as::<_, CommerceEntity>(
            "INSERT INTO dbo.commerces (alias, alias_type_id, legal_business_name, account_id,
                              ruc, commerce_status_id)
            OUTPUT INSERTED.*
             VALUES (@p1, @p2, @p3, @p4, @p5, 1);"
        )
            .bind(&commerce_entity.alias)
            .bind(&commerce_entity.alias_type_id)
            .bind(&commerce_entity.legal_business_name)
            .bind(&commerce_entity.account_id)
            .bind(&commerce_entity.ruc)
            .fetch_optional(&self.pool)
            .await
    }
}