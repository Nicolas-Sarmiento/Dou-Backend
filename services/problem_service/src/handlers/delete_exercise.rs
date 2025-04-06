use axum::{
    extract::{Path, Extension},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use tokio::fs;
use std::path::Path as StdPath;
use serde_json::json;
use axum::response::IntoResponse;

pub async fn delete_problem(
    Path(problem_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> impl IntoResponse {

    let row = match sqlx::query("SELECT problem_statement_url FROM problems WHERE problem_id = $1")
        .bind(problem_id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(row)) => row,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({ "error": "Problem not found" })),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Database error: {}", e) })),
            );
        }
    };

    let statement_url: String = row.get("problem_statement_url");
    let path_parts: Vec<&str> = statement_url.split('/').collect();
    let uuid_folder = path_parts.get(3).map(|s| *s).unwrap_or_default();

    let full_path = format!("/app/problems/{}", uuid_folder);

    if StdPath::new(&full_path).exists() {
        if let Err(e) = fs::remove_dir_all(&full_path).await {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("Failed to delete problem folder: {}", e) })),
            );
        }
    }

    match sqlx::query("DELETE FROM problems WHERE problem_id = $1")
        .bind(problem_id)
        .execute(&pool)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Problem deleted successfully" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Failed to delete from database: {}", e) })),
        ),
    }
}
