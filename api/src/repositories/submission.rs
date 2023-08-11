use crate::{
    error::{ApiError, Result},
    graph::*,
};
use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct Submission {
    pub id: i32,
    pub contest_name: String,
    pub graph_name: String,
    pub user_id: String,
    pub user_name: Option<String>,
    pub user_nickname: Option<String>,
    pub score: Option<f64>,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct SubmissionContent {
    pub content: sqlx::types::Json<SubmissionData>,
}

#[derive(Serialize)]
pub struct SubmissionMetrics {
    pub metrics: sqlx::types::Json<MetricsData>,
}

#[derive(Serialize)]
pub struct CreateSubmission {
    pub contest_name: String,
    pub graph_name: String,
    pub user_id: String,
    pub content: sqlx::types::Json<SubmissionData>,
    pub metrics: sqlx::types::Json<MetricsData>,
}

#[async_trait]
pub trait SubmissionRepository: Clone + Send + Sync + 'static {
    async fn find(&self, submission_id: i32) -> Result<Submission>;
    async fn find_content(&self, submission_id: i32) -> Result<SubmissionData>;
    async fn find_all(&self, constest_name: &str, graph_name: &str) -> Result<Vec<Submission>>;
    async fn find_standings(&self, contest_name: &str, graph_name: &str)
        -> Result<Vec<Submission>>;
    async fn find_all_by_user_id(
        &self,
        contest_name: &str,
        graph_name: &str,
        user_id: &str,
    ) -> Result<Vec<Submission>>;
    async fn create(&self, submission: CreateSubmission) -> Result<()>;
}

#[derive(Clone)]
pub struct SubmissionRepositoryForDB {
    pool: PgPool,
}

impl SubmissionRepositoryForDB {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryForDB {
    async fn find(&self, submission_id: i32) -> Result<Submission> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(Submission, "sql/submissions/find.sql", submission_id)
            .fetch_one(&mut conn)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
                _ => ApiError::Unknown("error".into()),
            })
    }

    async fn find_content(&self, submission_id: i32) -> Result<SubmissionData> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            SubmissionContent,
            "sql/submissions/find_content.sql",
            submission_id
        )
        .fetch_one(&mut conn)
        .await
        .map(|record| record.content.0)
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => ApiError::NotFound("record not found".into()),
            _ => ApiError::Unknown("error".into()),
        })
    }

    async fn find_all(&self, contest_name: &str, graph_name: &str) -> Result<Vec<Submission>> {
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

    async fn find_standings(
        &self,
        contest_name: &str,
        graph_name: &str,
    ) -> Result<Vec<Submission>> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file_as!(
            Submission,
            "sql/submissions/find_standings.sql",
            contest_name,
            graph_name,
        )
        .fetch_all(&mut conn)
        .await
        .map_err(|e| ApiError::Unknown(e.to_string().into()))
    }

    async fn find_all_by_user_id(
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

    async fn create(&self, submission: CreateSubmission) -> Result<()> {
        let mut conn = super::connection(&self.pool).await?;
        sqlx::query_file!(
            "sql/submissions/save.sql",
            submission.contest_name,
            submission.graph_name,
            submission.user_id,
            json!(submission.content),
            json!(submission.metrics)
        )
        .execute(&mut conn)
        .await
        .map(|_| ())
        .map_err(|e| ApiError::Unknown(e.to_string()))
    }
}

#[derive(Clone)]
struct SubmissionRepositoryForMemory;

#[async_trait]
impl SubmissionRepository for SubmissionRepositoryForMemory {
    async fn find(&self, _submission_id: i32) -> Result<Submission> {
        unimplemented!("unimplemented!")
    }

    async fn find_content(&self, _submission_id: i32) -> Result<SubmissionData> {
        unimplemented!("unimplemented!")
    }

    async fn find_all(&self, _constest_name: &str, _graph_name: &str) -> Result<Vec<Submission>> {
        unimplemented!("unimplemented!")
    }

    async fn find_standings(
        &self,
        _contest_name: &str,
        _graph_name: &str,
    ) -> Result<Vec<Submission>> {
        unimplemented!("unimplemented!")
    }

    async fn find_all_by_user_id(
        &self,
        _contest_name: &str,
        _graph_name: &str,
        _user_id: &str,
    ) -> Result<Vec<Submission>> {
        unimplemented!("unimplemented!")
    }

    async fn create(&self, _submission: CreateSubmission) -> Result<()> {
        unimplemented!("unimplemented!")
    }
}
