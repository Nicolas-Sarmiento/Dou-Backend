use axum::{Extension, extract::Path, http::StatusCode};
use sqlx::PgPool;

pub async fn delete_user(
    Extension(pool): Extension<PgPool>,
    Path(user_id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!(
        "DELETE FROM users WHERE user_id = $1",
        user_id
    )
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
