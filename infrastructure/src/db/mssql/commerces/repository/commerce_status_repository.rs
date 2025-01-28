use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{AnyPool, Error};
use crate::db::mssql::commerces::entity::commerce_status_entity::CommerceStatusEntity;

#[async_trait]
pub trait CommerceStatusRepository {
    async fn find_commerce_status_by_id(&self,
                                commerce_status_id: &i64)
        -> Result<Option<CommerceStatusEntity>, Error>;
}

pub struct SqlxCommerceStatusRepository {
    pool: Arc<AnyPool>,
}

impl SqlxCommerceStatusRepository {
    pub fn new(pool: Arc<AnyPool>) -> Self {
        Self { pool }
    }
}

impl CommerceStatusRepository for SqlxCommerceStatusRepository {
    async fn find_commerce_status_by_id(&self, commerce_status_id: &i64)
        -> Result<Option<CommerceStatusEntity>, Error> {
        sqlx::query_as::<_, CommerceStatusEntity>(
            "SELECT * FROM dbo.commerce_status WHERE commerce_status_id = @p1"
        )
            .bind(commerce_status_id)
            .fetch_optional(&self.pool)
            .await
    }
}