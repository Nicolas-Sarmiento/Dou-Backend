use axum::{extract::{Path, Extension}, Json};
use sqlx::PgPool;
use crate::models::{User, CreateUser};

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, axum::http::StatusCode> {
    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, user_password, user_email, user_role) 
        VALUES ($1, $2, $3, $4) RETURNING user_id, username, user_email, user_role",
        payload.username,
        payload.user_password, 
        payload.user_email,
        payload.user_role
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
