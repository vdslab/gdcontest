use crate::{
    error::{ApiError, Result},
    models::AdminUser,
};
use sqlx::PgPool;

pub struct AdminUserRepository {
    pool: PgPool,
}

impl AdminUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, user_id: &str) -> Result<AdminUser> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(AdminUser, "sql/admin_users/find.sql", user_id)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }
}
