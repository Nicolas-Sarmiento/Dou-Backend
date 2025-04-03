use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::PgPool;
use sqlx::Row;
use bcrypt::verify;
use crate::models::user_auth::User;
use crate::models::user_auth::LoginRequest;
use crate::models::user_auth::Claims;
use crate::models::user_auth::TokenResponse;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;



pub async fn auth_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, StatusCode> {

    let q = "SELECT u.USER_ID, u.USERNAME, u.USER_PASSWORD ,u.USER_EMAIL, r.USER_ROLE_NAME 
            FROM users u JOIN user_roles r on u.user_role = r.user_role_id
            WHERE u.username = $1";

    let row = sqlx::query(q)
        .bind(payload.username)
        .fetch_optional(&pool)
        .await
    	.map_err( |e| {
            eprintln!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    
    let row = row.ok_or(StatusCode::NOT_FOUND)?;
    let hashed_password:String = row.get("user_password");
    match verify( payload.user_password, &hashed_password) {
        Ok(valid) if valid => {

            let user = User{
                user_id: row.get("user_id"),
                username: row.get("username"),
                user_email : row.get("user_email"),
                user_role : row.get("user_role_name"),
            };

            let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET not defined!");

            let expiration = Utc::now()
                .checked_add_signed(Duration::days(1))
                .expect("Expiration time error!")
                .timestamp() as usize;

            let claims = Claims {
                sub : user.user_id.to_string(),
                name: user.username.clone(),
                email: user.user_email.clone(),
                role : user.user_role.clone(),
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret_key.as_ref()),
            ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;



            Ok(Json(TokenResponse{ token, user}))
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }

}