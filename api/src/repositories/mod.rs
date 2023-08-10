mod contest;
mod graph;
mod submission;
mod user;
pub use contest::*;
pub use graph::*;
pub use submission::*;
pub use user::*;

use crate::error::{ApiError, Result};
use sqlx::{PgConnection, PgPool};

async fn connection(pool: &PgPool) -> Result<PgConnection> {
    pool.acquire()
        .await
        .map(|conn| conn.detach())
        .map_err(|_| ApiError::Unknown("error".into()))
}
