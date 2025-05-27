use axum::{
    extract::{Extension, Multipart},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::{PgPool, Row};
use tokio::fs;
use uuid::Uuid;
use std::path::Path;

use serde_json::json;
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine as _;

use crate::models::models::{MaterialResponse, AttachmentResponse};
use crate::utils::auth::AuthenticatedUser;

pub async fn create_material(
    Extension(pool): Extension<PgPool>,
    AuthenticatedUser(claims): AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {

    if claims.role != "PROFESSOR" {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Unauthorized: only admins can create materials" })),
        ));
    }
    
    let row = sqlx::query("INSERT INTO materials (description_path) VALUES ('') RETURNING material_id")
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Database error on insert: {}", e)})),
            )
        })?;

    let material_id: i32 = row.get("material_id");
    let dir_path = format!("/app/materials/{}", material_id);

    fs::create_dir_all(&dir_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create directory: {}", e)})),
        )
    })?;

    let mut description_path: Option<String> = None;
    let mut attachments: Vec<AttachmentResponse> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Failed to read field: {}", e)})),
        )
    })? {
        let file_name = field.file_name().unwrap_or("").to_string();
        let data = field.bytes().await.map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Failed to read file: {}", e)})),
            )
        })?;

        if data.is_empty() {
            continue;
        }

        let save_path = format!("{}/{}", dir_path, file_name);

        fs::write(&save_path, &data).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to save file: {}", e)})),
            )
        })?;

        if file_name.to_lowercase().ends_with(".txt") && description_path.is_none() {
            description_path = Some(save_path.clone());

            sqlx::query("UPDATE materials SET description_path = $1 WHERE material_id = $2")
                .bind(&save_path)
                .bind(material_id)
                .execute(&pool)
                .await
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({"error": format!("Failed to update material: {}", e)})),
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

    if description_path.is_none() {
        let _ = fs::remove_dir_all(&dir_path).await;
        let _ = sqlx::query("DELETE FROM materials WHERE material_id = $1")
            .bind(material_id)
            .execute(&pool)
            .await;

        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Material must include at least one .txt file as description."})),
        ));
    }


    let response = MaterialResponse {
        material_id,
        description_path: description_path.unwrap(),
        attachments,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
