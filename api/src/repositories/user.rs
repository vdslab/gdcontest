use crate::error::{ApiError, Result};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize, Serialize)]
pub struct User {
    pub user_id: String,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub email: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct UpsertUser {
    pub user_id: String,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Clone + Send + Sync + 'static {
    async fn save(&self, user: UpsertUser) -> Result<User>;
}

#[derive(Clone)]
pub struct UserRepositoryForDB {
    pool: PgPool,
}

impl UserRepositoryForDB {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryForDB {
    async fn save(&self, user: UpsertUser) -> Result<User> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            User,
            "sql/users/save.sql",
            user.user_id,
            user.name,
            user.nickname,
            user.email,
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ApiError::Unknown(e.to_string()))
    }
}
