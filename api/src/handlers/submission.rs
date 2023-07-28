use crate::{auth::validate_token, error::Result, models::*, repositories::*};
use axum::{extract::Path, Extension, Json};
use axum_auth::AuthBearer;
use std::sync::Arc;

pub async fn list(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Vec<Submission>>> {
    submissions
        .find_all(&contest_name, &graph_name)
        .await
        .map(|records| Json(records))
}

pub async fn list_standings(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Vec<StandingsSubmission>>> {
    submissions
        .find_standings(&contest_name, &graph_name)
        .await
        .map(|records| Json(records))
}

pub async fn get(
    Path(submission_id): Path<i32>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Submission>> {
    submissions
        .find(submission_id)
        .await
        .map(|record| Json(record))
}

pub async fn post(
    Path((contest_name, graph_name)): Path<(String, String)>,
    AuthBearer(token): AuthBearer,
    Extension(graphs): Extension<Arc<AdminGraphRepository>>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
    Json(content): Json<SubmissionData>,
) -> Result<Json<Submission>> {
    let user_id = validate_token(&token).await?;
    let graph_content = graphs.find_content(&contest_name, &graph_name).await?;
    let graph_distance =
        distance_from_bytes(graphs.find_distance(&contest_name, &graph_name).await?);
    let metrics = compute_metrics(&graph_content, &graph_distance, &content);
    submissions
        .save(contest_name, graph_name, user_id, content, metrics)
        .await
        .map(|record| Json(record))
}

pub async fn list_by_user(
    Path((contest_name, graph_name, user_id)): Path<(String, String, String)>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Vec<Submission>>> {
    submissions
        .find_all_by_user(&contest_name, &graph_name, &user_id)
        .await
        .map(|records| Json(records))
}
