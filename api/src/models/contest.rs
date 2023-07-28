use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Contest {
    pub contest_name: String,
    pub start_at: sqlx::types::chrono::NaiveDateTime,
    pub end_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct AdminContest {
    pub contest_name: String,
    pub published: bool,
    pub start_at: sqlx::types::chrono::NaiveDateTime,
    pub end_at: sqlx::types::chrono::NaiveDateTime,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub updated_at: sqlx::types::chrono::NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct AdminUpsertContest {
    pub published: bool,
    pub start_at: sqlx::types::chrono::NaiveDateTime,
    pub end_at: sqlx::types::chrono::NaiveDateTime,
}
