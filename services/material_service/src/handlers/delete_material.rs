use axum::{extract::{Extension, Path}, http::StatusCode};
use sqlx::PgPool;
use std::fs;
use std::path::Path;

pub async fn delete_material(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query!("DELETE FROM attachments WHERE material_id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = sqlx::query!("DELETE FROM materials WHERE material_id = $1", id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let dir_path = format!("./material{}", id);
    if Path::new(&dir_path).exists() {
        fs::remove_dir_all(&dir_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}
