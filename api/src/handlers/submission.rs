use crate::{auth::Validator, error::Result, graph::*, repositories::*};
use axum::{extract::Path, Extension, Json};
use axum_auth::{AuthBasic, AuthBearer};
use std::sync::Arc;

pub async fn list<R>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(submissions): Extension<Arc<R>>,
) -> Result<Json<Vec<Submission>>>
where
    R: SubmissionRepository,
{
    submissions
        .find_all(&contest_name, &graph_name)
        .await
        .map(|records| Json(records))
}

pub async fn list_standings<R>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(submissions): Extension<Arc<R>>,
) -> Result<Json<Vec<Submission>>>
where
    R: SubmissionRepository,
{
    submissions
        .find_standings(&contest_name, &graph_name)
        .await
        .map(|records| Json(records))
}

pub async fn get<R>(
    Path(submission_id): Path<i32>,
    Extension(submissions): Extension<Arc<R>>,
) -> Result<Json<Submission>>
where
    R: SubmissionRepository,
{
    submissions
        .find(submission_id)
        .await
        .map(|record| Json(record))
}

pub async fn post<R1, R2, V>(
    Path((contest_name, graph_name)): Path<(String, String)>,
    AuthBearer(token): AuthBearer,
    Extension(submissions): Extension<Arc<R1>>,
    Extension(graphs): Extension<Arc<R2>>,
    Extension(validator): Extension<Arc<V>>,
    Json(content): Json<SubmissionData>,
) -> Result<()>
where
    V: Validator,
    R1: SubmissionRepository,
    R2: GraphRepository,
{
    let user_id = validator.validate_token(token).await?;
    let graph_content = graphs.find_content(&contest_name, &graph_name).await?;
    let graph_distance = graphs.find_distance(&contest_name, &graph_name).await?;
    let metrics = compute_metrics(&graph_content, &graph_distance, &content);
    let submission = CreateSubmission {
        contest_name,
        graph_name,
        user_id,
        content: sqlx::types::Json(content),
        metrics: sqlx::types::Json(metrics),
    };
    submissions.create(submission).await
}

pub async fn list_by_user<R>(
    Path((contest_name, graph_name, user_id)): Path<(String, String, String)>,
    Extension(submissions): Extension<Arc<R>>,
) -> Result<Json<Vec<Submission>>>
where
    R: SubmissionRepository,
{
    submissions
        .find_all_by_user_id(&contest_name, &graph_name, &user_id)
        .await
        .map(|records| Json(records))
}

pub async fn admin_get_content<R, V>(
    Path(submission_id): Path<i32>,
    Extension(submissions): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<SubmissionData>>
where
    R: SubmissionRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    submissions
        .find_content(submission_id)
        .await
        .map(|record| Json(record))
}
