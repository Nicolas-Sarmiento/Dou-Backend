use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{User, CreateUser};

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    // Hashear la contraseña
    let hashed_password = hash(payload.user_password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (username, user_password, user_email, user_role) 
        VALUES ($1, $2, $3, 1) RETURNING user_id, username, user_email, user_role",
        payload.username,
        hashed_password,
        payload.user_email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}
