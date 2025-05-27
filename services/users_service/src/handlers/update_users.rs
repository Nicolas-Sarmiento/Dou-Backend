use axum::{extract::{Extension, Path}, Json, http::StatusCode};
use sqlx::{PgPool, Row};
use bcrypt::{verify, hash, DEFAULT_COST};
use crate::models::{User, UpdateUser, UpdatePasswordRequest};
use crate::utils::validations::{validate_username, validate_password};
use crate::utils::auth::AuthenticatedUser;

pub async fn update_user(
    AuthenticatedUser(claims): AuthenticatedUser,
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let is_self = claims.sub == user_id.to_string();
    let is_admin = claims.role == "PROFESSOR";

    if !is_self && !is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !validate_username(&payload.username) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let row = sqlx::query("SELECT user_password FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let stored_hash: String = row.get("user_password");

    let password_matches = verify(&payload.user_password, &stored_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !password_matches {
        return Err(StatusCode::UNAUTHORIZED); 
    }

    let query = "
        UPDATE users 
        SET username = $1, user_email = $2, user_role = $3
        WHERE user_id = $4
        RETURNING user_id, username, user_email, user_role
    ";

    let updated_user = sqlx::query(query)
        .bind(payload.username)
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


pub async fn update_user_password(
    AuthenticatedUser(claims): AuthenticatedUser,
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdatePasswordRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {

    let is_self = claims.sub == user_id.to_string();
    let is_admin = claims.role == "PROFESSOR";

    if !is_self && !is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !validate_password(&payload.new_password) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let row = sqlx::query("SELECT user_password FROM users WHERE user_id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let stored_hash: String = row.get("user_password");

    let password_matches = verify(&payload.current_password, &stored_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !password_matches {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let new_hash = hash(&payload.new_password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query(
        "UPDATE users SET user_password = $1 WHERE user_id = $2 RETURNING user_id, username, user_email, user_role"
    )
        .bind(new_hash)
        .bind(user_id)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(row) => {
            let user_response = User {
                user_id: row.get("user_id"),
                username: row.get("username"),
                user_password: "{}".to_string(),
                user_email: row.get("user_email"),
                user_role: row.get("user_role"),
            };
            Ok((StatusCode::OK, Json(user_response)))
        },
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


