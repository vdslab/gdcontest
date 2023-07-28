use crate::{
    error::{ApiError, Result},
    models::*,
};
use serde_json::json;
use sqlx::PgPool;

pub struct SubmissionRepository {
    pool: PgPool,
}

impl SubmissionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, submission_id: i32) -> Result<Submission> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Submission, "sql/submissions/find.sql", submission_id)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }

    pub async fn find_all(&self, contest_name: &str, graph_name: &str) -> Result<Vec<Submission>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            Submission,
            "sql/submissions/find_all.sql",
            contest_name,
            graph_name,
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn find_standings(
        &self,
        contest_name: &str,
        graph_name: &str,
    ) -> Result<Vec<StandingsSubmission>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            StandingsSubmission,
            "sql/submissions/find_standings.sql",
            contest_name,
            graph_name,
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn find_all_by_user(
        &self,
        contest_name: &str,
        graph_name: &str,
        user_id: &str,
    ) -> Result<Vec<Submission>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            Submission,
            "sql/submissions/find_all_by_user.sql",
            contest_name,
            graph_name,
            user_id
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|_| ApiError::Unknown("error".into()))
    }

    pub async fn save(
        &self,
        contest_name: String,
        graph_name: String,
        user_id: String,
        content: SubmissionData,
        metrics: MetricsData,
    ) -> Result<Submission> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            Submission,
            "sql/submissions/save.sql",
            contest_name,
            graph_name,
            user_id,
            json!(content),
            json!(metrics)
        )
        .fetch_one(&mut conn)
        .await
        .map_err(|e| ApiError::Unknown(e.to_string()))
    }
}
