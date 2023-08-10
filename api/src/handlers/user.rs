use crate::{auth::Validator, error::Result, repositories::*};
use axum::{Extension, Json};
use axum_auth::AuthBasic;
use std::sync::Arc;

pub async fn admin_put<R, V>(
    Extension(users): Extension<Arc<R>>,
    Extension(validator): Extension<Arc<V>>,
    AuthBasic(auth): AuthBasic,
    Json(user): Json<UpsertUser>,
) -> Result<Json<User>>
where
    R: UserRepository,
    V: Validator,
{
    validator.validate_user(auth).await?;
    users.save(user).await.map(|record| Json(record))
}
