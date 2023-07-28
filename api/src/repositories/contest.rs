use crate::{
    error::{ApiError, Result},
    models::*,
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
}

pub struct AdminContestRepository {
    pool: PgPool,
}

impl AdminContestRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, contest_name: &str) -> Result<AdminContest> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(AdminContest, "sql/contests/admin_find.sql", contest_name)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }

    pub async fn find_all(&self) -> Result<Vec<AdminContest>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(AdminContest, "sql/contests/admin_find_all.sql")
            .fetch_all(&mut conn)
            .await
            .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn save(
        &self,
        contest_name: &str,
        contest: &AdminUpsertContest,
    ) -> Result<AdminContest> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            AdminContest,
            "sql/contests/admin_save.sql",
            contest_name,
            contest.published,
            contest.start_at,
            contest.end_at
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn delete(&self, contest_name: &str) -> Result<()> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file!("sql/contests/admin_delete.sql", contest_name,)
            .execute(&mut conn)
            .await
            .map(|_| ())
            .map_err(|_| ApiError::Unknown("error".into()))
    }
}
