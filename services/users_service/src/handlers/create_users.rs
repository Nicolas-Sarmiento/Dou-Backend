use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::{PgPool, Row};
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{User, CreateUser};
use crate::utils::validations::{validate_username, validate_password};

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {

    if !validate_username(&payload.username) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !validate_password(&payload.user_password) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hashed_password = hash(payload.user_password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query = "
        INSERT INTO users (username, user_password, user_email, user_role) 
        VALUES ($1, $2, $3, $4) 
        RETURNING user_id, username, user_email, user_role
    ";

    let created_user = sqlx::query(query)
        .bind(payload.username)
        .bind(hashed_password.clone())
        .bind(payload.user_email)
        .bind(1)
        .fetch_one(&pool)
        .await;

    match created_user {
        Ok(created_user) => {
            let user_response = User {
                user_id: created_user.get("user_id"),
                username: created_user.get("username"),
                user_password: "{}".to_string(),
                user_email: created_user.get("user_email"),
                user_role: created_user.get("user_role"),
            };
            Ok((StatusCode::CREATED, Json(user_response)))
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().map_or(false, |code| code == "23505") => {
            Err(StatusCode::CONFLICT)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
