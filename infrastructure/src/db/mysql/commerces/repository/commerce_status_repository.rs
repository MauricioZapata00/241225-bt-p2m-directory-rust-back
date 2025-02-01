use std::sync::Arc;
use async_trait::async_trait;
use sqlx::{Error, MySqlPool};
use crate::db::mysql::commerces::entity::commerce_status_entity::CommerceStatusEntity;

#[async_trait]
pub trait CommerceStatusRepository {
    async fn find_commerce_status_by_id(&self,
                                commerce_status_id: &i64)
        -> Result<Option<CommerceStatusEntity>, Error>;
}

pub struct SqlxCommerceStatusRepository {
    pool: Arc<MySqlPool>,
}

impl SqlxCommerceStatusRepository {
    pub fn new(pool: Arc<MySqlPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommerceStatusRepository for SqlxCommerceStatusRepository {
    async fn find_commerce_status_by_id(&self, commerce_status_id: &i64)
                                        -> Result<Option<CommerceStatusEntity>, Error> {
        sqlx::query_as::<_, CommerceStatusEntity>(
            "SELECT * FROM commerce_status WHERE commerce_status_id = ? FOR UPDATE"
        )
            .bind(commerce_status_id)
            .fetch_optional(&*self.pool)
            .await
    }
}