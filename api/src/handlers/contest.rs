use crate::{auth::Validator, error::Result, models::*, repositories::*};
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

pub async fn admin_list<V>(
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<Vec<AdminContest>>>
where
    V: Validator,
{
    validator.validate_user(auth).await?;
    contests.find_all().await.map(|records| Json(records))
}

pub async fn admin_get<V>(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<Json<AdminContest>>
where
    V: Validator,
{
    validator.validate_user(auth).await?;
    contests
        .find(&contest_name)
        .await
        .map(|record| Json(record))
}

pub async fn admin_put<V>(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
    Json(contest): Json<AdminUpsertContest>,
) -> Result<Json<AdminContest>>
where
    V: Validator,
{
    validator.validate_user(auth).await?;
    contests
        .save(&contest_name, &contest)
        .await
        .map(|record| Json(record))
}

pub async fn admin_delete<V>(
    Path(contest_name): Path<String>,
    Extension(contests): Extension<Arc<AdminContestRepository>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
) -> Result<()>
where
    V: Validator,
{
    validator.validate_user(auth).await?;
    contests.delete(&contest_name).await
}
