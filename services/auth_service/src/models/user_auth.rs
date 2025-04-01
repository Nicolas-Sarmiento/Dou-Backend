use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub user_email: String,
    pub user_role: String,
}

#[derive (Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub user_password : String,
}