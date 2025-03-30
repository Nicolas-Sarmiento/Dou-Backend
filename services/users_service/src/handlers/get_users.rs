use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::PgPool;
use sqlx::Row;
use crate::models::User;

pub async fn get_users(
    Extension(pool): Extension<PgPool>
) -> Result<Json<Vec<User>>, StatusCode> {
    let q = "SELECT user_id, username, user_password, user_email, user_role FROM users";
    let query = sqlx::query(q);
    let rows= query.fetch_all(&pool)
    .await
    .map_err( |e| {
        eprintln!("Database error: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let users = rows.iter().map( |row| {
        User{
            user_id: row.get("user_id"),
            username: row.get("username"),
            user_password: row.get("user_password"),
            user_email : row.get("user_email"),
            user_role : row.get("user_role"),
        }
    }).collect();
    Ok(Json(users))
}