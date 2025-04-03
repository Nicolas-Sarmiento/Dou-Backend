use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, 
    pub name: String,
    pub email : String,
    pub role: String,
    pub exp: usize,  
}