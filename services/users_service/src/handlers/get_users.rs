use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::PgPool;
use crate::models::User;

pub async fn get_users(
    Extension(pool): Extension<PgPool>
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(
        User,
        "SELECT user_id, username, user_email, user_role FROM users"
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}
