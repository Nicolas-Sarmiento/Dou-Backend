use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::PgPool;
use sqlx::Row;
use bcrypt::verify;
use crate::models::user_auth::User;
use crate::models::user_auth::LoginRequest;

pub async fn auth_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<User>, StatusCode> {

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
    println!("{}",hashed_password);
    match verify( payload.user_password, &hashed_password) {
        Ok(valid) if valid => {
            let user = User{
                user_id: row.get("user_id"),
                username: row.get("username"),
                user_email : row.get("user_email"),
                user_role : row.get("user_role_name"),
            };
            Ok(Json(user))
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}