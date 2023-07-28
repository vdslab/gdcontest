use crate::{auth::validate_user, error::Result, models::*, repositories::*};
use axum::{extract::Path, Extension, Json};
use axum_auth::AuthBasic;
use std::sync::Arc;

pub async fn list(
    Extension(contests): Extension<Arc<ContestRepository>>,
) -> Result<Json<Vec<Contest>>> {
    contests.find_all().await.map(|records| Json(records))
}

pub async fn get(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<ContestRepository>>,
) -> Result<Json<Contest>> {
    contests
        .find(&contest_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_list(
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<Json<Vec<AdminContest>>> {
    validate_user(user, password).await?;
    contests.find_all().await.map(|records| Json(records))
}

pub async fn admin_get(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<Json<AdminContest>> {
    validate_user(user, password).await?;
    contests
        .find(&contest_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_put(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    AuthBasic((user, password)): AuthBasic,
    Json(contest): Json<AdminUpsertContest>,
) -> Result<Json<AdminContest>> {
    validate_user(user, password).await?;
    contests
        .save(&contest_name, &contest)
        .await
        .map(|record| Json(record))
}

pub async fn admin_delete(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    AuthBasic((user, password)): AuthBasic,
) -> Result<()> {
    validate_user(user, password).await?;
    contests.delete(&contest_name).await
}
