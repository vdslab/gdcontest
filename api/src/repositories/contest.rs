use crate::{
    error::{ApiError, Result},
    models::Contest,
};
use sqlx::PgPool;

pub struct ContestRepository {
    pool: PgPool,
}

impl ContestRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, contest_name: &str) -> Result<Contest> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Contest, "sql/contests/find.sql", contest_name)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }

    pub async fn find_all(&self) -> Result<Vec<Contest>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Contest, "sql/contests/find_all.sql")
            .fetch_all(&mut conn)
            .await
            .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn save(&self, contest: &Contest) -> Result<Contest> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            Contest,
            "sql/contests/save.sql",
            contest.contest_name,
            contest.is_public
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|_| ApiError::Unknown("error".into()))
    }
}
