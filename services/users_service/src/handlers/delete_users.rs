use axum::{extract::{Extension, Path}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Serialize;
use crate::models::{User, DeleteResponse};
use crate::utils::auth::AuthenticatedUser;

pub async fn delete_user(
    AuthenticatedUser(claims): AuthenticatedUser,
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, Json<DeleteResponse>), StatusCode> {
    let is_self = claims.sub == user_id.to_string();
    let is_admin = claims.role == "PROFESSOR";

    if !is_self && !is_admin {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let query = "DELETE FROM users WHERE user_id = $1";

    let delete_user = sqlx::query(query)
        .bind(user_id)
        .execute(&pool)
        .await;

    match delete_user {
        Ok(res) if res.rows_affected() > 0 => {
            Ok((StatusCode::OK, Json(DeleteResponse { message: "Usuario eliminado correctamente".to_string() })))
        }
        Ok(_) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }

}
