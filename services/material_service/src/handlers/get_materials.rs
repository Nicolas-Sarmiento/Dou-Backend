use axum::{extract::{Extension, Path}, http::StatusCode, Json};
use sqlx::{PgPool, Row};
use tokio::{fs::read_to_string, fs::read};
use base64::engine::general_purpose::STANDARD as base64_engine;
use base64::Engine as _;


use crate::models::models::{MaterialResponse, AttachmentResponse};

pub async fn get_materials(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<MaterialResponse>>), StatusCode> {
    let query = "
        SELECT material_id, description_path
        FROM materials
        ORDER BY material_id ASC
    ";

    let rows = sqlx::query(query).fetch_all(&pool).await;

    match rows {
        Ok(rows) => {
            let mut materials = Vec::new();

            for row in rows {
                let material_id: i32 = row.get("material_id");
                let description_path: String = row.get("description_path");

                let description_content = match read_to_string(&description_path).await {
                    Ok(content) => content,
                    Err(_) => "[Error al leer el archivo de descripción]".to_string(),
                };

                let attachment_rows = sqlx::query(
                    "SELECT file_name, file_path FROM attachments WHERE material_id = $1"
                )
                .bind(material_id)
                .fetch_all(&pool)
                .await
                .unwrap_or_default();

                let mut attachments = Vec::new();

                for row in attachment_rows {
                    let file_name: String = row.get("file_name");
                    let file_path: String = row.get("file_path");

                    let base64_content = match read(&file_path).await {
                        Ok(bytes) => Some(base64_engine.encode(&bytes)),
                        Err(_) => None,
                    };

                    attachments.push(AttachmentResponse {
                        file_name,
                        base64_content,
                    });
                }

                materials.push(MaterialResponse {
                    material_id,
                    description_path: description_content,
                    attachments,
                });
            }

            Ok((StatusCode::OK, Json(materials)))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_material_by_id(
    Path(material_id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<MaterialResponse>), (StatusCode, Json<serde_json::Value>)> {
    let row = sqlx::query("SELECT description_path FROM materials WHERE material_id = $1")
        .bind(material_id)
        .fetch_optional(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("Database error: {}", e)})),
            )
        })?;

    let Some(row) = row else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "Material not found"})),
        ));
    };

    let description_path: String = row.get("description_path");

    let description_content = match read_to_string(&description_path).await {
        Ok(content) => content,
        Err(_) => "[Error al leer el archivo de descripción]".to_string(),
    };

    let attachment_rows = sqlx::query(
        "SELECT file_name, file_path FROM attachments WHERE material_id = $1"
    )
    .bind(material_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("Error loading attachments: {}", e)})),
        )
    })?;

    let mut attachments = Vec::new();

    for row in attachment_rows {
        let file_name: String = row.get("file_name");
        let file_path: String = row.get("file_path");

        let base64_content = match read(&file_path).await {
            Ok(bytes) => Some(base64_engine.encode(&bytes)),
            Err(_) => None,
        };

        attachments.push(AttachmentResponse {
            file_name,
            base64_content,
        });
    }

    let response = MaterialResponse {
        material_id,
        description_path: description_content,
        attachments,
    };

    Ok((StatusCode::OK, Json(response)))
}

