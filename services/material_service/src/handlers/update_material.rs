use axum::{
    extract::{Extension, Multipart, Path},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::{PgPool, Row};
use tokio::fs;
use std::path::Path as FilePath;
use serde_json::json;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine as _;

use crate::models::models::{MaterialResponse, AttachmentResponse};

pub async fn update_material(
    Path(material_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let existing = sqlx::query("SELECT material_id FROM materials WHERE material_id = $1")
        .bind(material_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Database error: {}", e)})),
        ))?;

    if existing.is_none() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Material not found"})),
        ));
    }

    let dir_path = format!("/app/materials/{}", material_id);

    if FilePath::new(&dir_path).exists() {
        fs::remove_dir_all(&dir_path).await.map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to delete old directory: {}", e)})),
        ))?;
    }

    sqlx::query("DELETE FROM attachments WHERE material_id = $1")
        .bind(material_id)
        .execute(&pool)
        .await
        .map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to delete old attachments: {}", e)})),
        ))?;

    fs::create_dir_all(&dir_path).await.map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"error": format!("Failed to create directory: {}", e)})),
    ))?;

    let mut description_path: Option<String> = None;
    let mut attachments: Vec<AttachmentResponse> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| (
        StatusCode::BAD_REQUEST,
        Json(json!({"error": format!("Failed to read field: {}", e)})),
    ))? {
        let name = field.name().unwrap_or("").to_string();
        let file_name = field.file_name().unwrap_or("file.dat").to_string();
        let data = field.bytes().await.map_err(|e| (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Failed to read file: {}", e)})),
        ))?;

        let save_path = format!("{}/{}", dir_path, file_name);

        fs::write(&save_path, &data).await.map_err(|e| (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to save file: {}", e)})),
        ))?;

        if name == "description" && description_path.is_none() {
            description_path = Some(save_path.clone());

            sqlx::query("UPDATE materials SET description_path = $1 WHERE material_id = $2")
                .bind(&save_path)
                .bind(material_id)
                .execute(&pool)
                .await
                .map_err(|e| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to update material: {}", e)})),
                ))?;
        } else {
            sqlx::query("INSERT INTO attachments (material_id, file_path, file_name) VALUES ($1, $2, $3)")
                .bind(material_id)
                .bind(&save_path)
                .bind(&file_name)
                .execute(&pool)
                .await
                .map_err(|e| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to insert attachment: {}", e)})),
                ))?;

            let base64_content = match fs::read(&save_path).await {
                Ok(bytes) => Some(base64_engine.encode(&bytes)),
                Err(_) => None,
            };

            attachments.push(AttachmentResponse {
                file_name,
                base64_content,
            });
        }
    }

    let description_path = match description_path {
        Some(path) => path,
        None => {
            let row = sqlx::query("SELECT description_path FROM materials WHERE material_id = $1")
                .bind(material_id)
                .fetch_one(&pool)
                .await
                .map_err(|e| (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": format!("Failed to retrieve material: {}", e)})),
                ))?;
            row.get("description_path")
        }
    };

    let response = MaterialResponse {
        material_id,
        description_path,
        attachments,
    };

    Ok((StatusCode::OK, Json(response)))
}

