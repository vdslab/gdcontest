use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Contest {
    pub contest_name: String,
    pub is_public: bool,
}

#[derive(Deserialize, Serialize)]
pub struct UpsertContest {
    pub is_public: bool,
}
