use axum::{extract::{Extension, Path}, Json, http::StatusCode};
use sqlx::{PgPool, Row};
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{User, UpdateUser};
use crate::utils::validations::{validate_username, validate_password};

pub async fn update_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUser>,
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
        UPDATE users 
        SET username = $1, user_password = $2, user_email = $3, user_role = $4
        WHERE user_id = $5
        RETURNING user_id, username, user_email, user_role
    ";

    let updated_user = sqlx::query(query)
        .bind(payload.username)
        .bind(hashed_password)
        .bind(payload.user_email)
        .bind(payload.user_role)
        .bind(user_id)
        .fetch_one(&pool)
        .await;

    match updated_user {
        Ok(row) => {
            let user_response = User {
                user_id: row.get("user_id"),
                username: row.get("username"),
                user_password: "{}".to_string(),
                user_email: row.get("user_email"),
                user_role: row.get("user_role"),
            };
            Ok((StatusCode::OK, Json(user_response)))
        }
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}



