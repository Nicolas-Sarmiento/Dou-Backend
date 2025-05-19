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
    let dir_path = format!("/app/materials/{}", material_id);

    if !FilePath::new(&dir_path).exists() {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Material folder not found"})),
        ));
    }

    let current_description_path: Option<String> = sqlx::query("SELECT description_path FROM materials WHERE material_id = $1")
        .bind(material_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to query current description: {}", e)})),
            )
        })?
        .and_then(|row| row.try_get("description_path").ok());

    let mut description_updated = false;

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Failed to read field: {}", e)})),
        )
    })? {
        let name = field.name().unwrap_or("").to_string();
        let mut file_name = field.file_name().unwrap_or("file.dat").to_string();

        if name == "description" {
            file_name = "description.txt".to_string();
        }

        let data = field.bytes().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Failed to read file: {}", e)})),
            )
        })?;

        let save_path = format!("{}/{}", dir_path, file_name);

        fs::write(&save_path, &data).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to save file: {}", e)})),
            )
        })?;

        if name == "description" {
            sqlx::query("UPDATE materials SET description_path = $1 WHERE material_id = $2")
                .bind(&save_path)
                .bind(material_id)
                .execute(&pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to update description path: {}", e)})),
                    )
                })?;

            description_updated = true;
        } else {
            let existing = sqlx::query("SELECT attachment_id FROM attachments WHERE material_id = $1 AND file_name = $2")
                .bind(material_id)
                .bind(&file_name)
                .fetch_optional(&pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Database error: {}", e)})),
                    )
                })?;

            if let Some(row) = existing {
                let attachment_id: i32 = row.get("attachment_id");

                sqlx::query("UPDATE attachments SET file_path = $1 WHERE attachment_id = $2")
                    .bind(&save_path)
                    .bind(attachment_id)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": format!("Failed to update attachment path: {}", e)})),
                        )
                    })?;
            } else {
                sqlx::query("INSERT INTO attachments (material_id, file_path, file_name) VALUES ($1, $2, $3)")
                    .bind(material_id)
                    .bind(&save_path)
                    .bind(&file_name)
                    .execute(&pool)
                    .await
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({"error": format!("Failed to insert attachment: {}", e)})),
                        )
                    })?;
            }
        }
    }

    if !description_updated {
        if current_description_path.is_none() || current_description_path.clone().unwrap_or_default().is_empty() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing required file: description"})),
            ));
        }
    }

    Ok((StatusCode::OK, Json(json!({"message": "Material updated successfully"}))))
}


