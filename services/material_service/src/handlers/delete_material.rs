use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::PgPool;
use tokio::fs;
use std::path::Path as FilePath;
use serde_json::json;

pub async fn delete_material(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    sqlx::query("DELETE FROM attachments WHERE material_id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete attachments: {}", e) })),
            )
        })?;

    let result = sqlx::query("DELETE FROM materials WHERE material_id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete material: {}", e) })),
            )
        })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("Material with id {} not found", id) })),
        ));
    }

    let dir_path = format!("/app/materials/{}", id);
    if FilePath::new(&dir_path).exists() {
        fs::remove_dir_all(&dir_path).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete directory: {}", e) })),
            )
        })?;
    }

    Ok((
        StatusCode::OK,
        Json(json!({ "message": "Material deleted successfully" })),
    ))
}
