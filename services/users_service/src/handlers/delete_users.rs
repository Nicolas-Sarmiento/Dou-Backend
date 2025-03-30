use axum::{extract::{Extension, Path}, Json, http::StatusCode};
use sqlx::PgPool;
use serde::Serialize;
use crate::models::{User, DeleteResponse};

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, Json<DeleteResponse>), StatusCode> {
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
