use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub user_password: String,
    pub user_email: String,
    pub user_role: i32,
}

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub user_password: String,
    pub user_email: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUser {
    pub username: String,
    pub user_password: String,
    pub user_email: String,
    pub user_role: i32,
}

#[derive(Deserialize, Serialize)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Serialize)]
pub struct DeleteResponse {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub name: String,
    pub email : String,
    pub role: String,
    pub exp: usize,  
}

