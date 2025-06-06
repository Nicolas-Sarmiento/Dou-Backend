use axum::{extract::{Extension,Path}, Json, http::StatusCode};
use sqlx::PgPool;
use sqlx::Row;
use crate::models::User;
use crate::utils::auth::AuthenticatedUser;

pub async fn get_users(
    Extension(pool): Extension<PgPool>,
    _auth: AuthenticatedUser,
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

pub async fn get_user_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    _auth: AuthenticatedUser,
) -> Result<Json<User>, StatusCode> {
    let q = "SELECT user_id, username, user_password, user_email, user_role FROM users WHERE user_id = $1";
    let query = sqlx::query(q);
    let row= query
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err( |e| {
            eprintln!("Database error: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
         })?;
    let user =User{
            user_id: row.get("user_id"),
            username: row.get("username"),
            user_password: row.get("user_password"),
            user_email : row.get("user_email"),
            user_role : row.get("user_role"),
        };
    Ok(Json(user))
}