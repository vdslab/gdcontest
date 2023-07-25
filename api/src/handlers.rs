use crate::{
    auth::validate_token,
    error::{ApiError, Result},
    models::*,
    repositories::*,
};
use axum::{extract::Path, Extension, Json};
use axum_auth::AuthBearer;
use std::sync::Arc;

pub async fn list_contests(
    Extension(contests): Extension<Arc<ContestRepository>>,
) -> Result<Json<Vec<Contest>>> {
    contests.find_all().await.map(|records| Json(records))
}

pub async fn get_contest(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<ContestRepository>>,
) -> Result<Json<Contest>> {
    contests
        .find(&contest_name)
        .await
        .map(|record| Json(record))
}

pub async fn put_contest(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<ContestRepository>>,
    Extension(admin_users): Extension<Arc<AdminUserRepository>>,
    AuthBearer(token): AuthBearer,
    Json(contest): Json<UpsertContest>,
) -> Result<Json<Contest>> {
    let user_id = validate_token(&token).await?;
    let _ = admin_users
        .find(&user_id)
        .await
        .map_err(|_| ApiError::Forbidden("unauthorized".into()))?;
    let contest = Contest {
        contest_name,
        is_public: contest.is_public,
    };
    contests.save(&contest).await.map(|record| Json(record))
}

pub async fn list_graphs(
    Path(contest_name): Path<String>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
) -> Result<Json<Vec<GraphMeta>>> {
    graphs
        .find_all(&contest_name)
        .await
        .map(|records| Json(records))
}

pub async fn get_graph(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
) -> Result<Json<GraphMeta>> {
    graphs
        .find_meta(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn get_graph_content(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
) -> Result<Json<GraphData>> {
    graphs
        .find_content(&contest_name, &graph_name)
        .await
        .map(|record| Json(record))
}

pub async fn put_graph(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(graphs): Extension<Arc<GraphRepository>>,
    Extension(admin_users): Extension<Arc<AdminUserRepository>>,
    AuthBearer(token): AuthBearer,
    Json(graph): Json<GraphData>,
) -> Result<Json<UpsertGraph>> {
    let user_id = validate_token(&token).await?;
    let _ = admin_users
        .find(&user_id)
        .await
        .map_err(|_| ApiError::Forbidden("unauthorized".into()))?;
    let graph = UpsertGraph::new(contest_name, graph_name, graph);
    graphs.save(&graph).await.map(|record| Json(record))
}

pub async fn list_submissions(
    Path((contest_name, graph_name)): Path<(String, String)>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Vec<Submission>>> {
    submissions
        .find_all(&contest_name, &graph_name)
        .await
        .map(|records| Json(records))
}

pub async fn post_submission(
    Path((contest_name, graph_name)): Path<(String, String)>,
    AuthBearer(token): AuthBearer,
    Extension(graphs): Extension<Arc<GraphRepository>>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
    Json(content): Json<SubmissionData>,
) -> Result<Json<Submission>> {
    let user_id = validate_token(&token).await?;
    let graph = graphs.find(&contest_name, &graph_name).await?;
    let submission = CreateSubmission::new(graph, user_id, content);
    submissions
        .save(&submission)
        .await
        .map(|record| Json(record))
}

pub async fn get_submission(
    Path(submission_id): Path<i32>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Submission>> {
    submissions
        .find(submission_id)
        .await
        .map(|record| Json(record))
}

pub async fn list_user_submissions(
    Path((contest_name, graph_name, user_id)): Path<(String, String, String)>,
    Extension(submissions): Extension<Arc<SubmissionRepository>>,
) -> Result<Json<Vec<Submission>>> {
    submissions
        .find_all_by_user(&contest_name, &graph_name, &user_id)
        .await
        .map(|records| Json(records))
}
