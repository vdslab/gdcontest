use serde::Serialize;

#[derive(Serialize)]
pub struct AdminUser {
    pub user_id: String,
}
