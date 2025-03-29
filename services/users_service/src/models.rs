use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub user_email: String,
    pub user_role: i32,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub user_password: String,
    pub user_email: String,
}