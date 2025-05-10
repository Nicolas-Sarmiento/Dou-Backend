use axum::{
    extract::{Extension, Multipart},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use std::{fs, path::PathBuf};
use crate::models::{MaterialResponse, AttachmentResponse};

pub async fn upload_material(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<MaterialResponse>), StatusCode> {
    let material = sqlx::query("INSERT INTO materials (description_path) VALUES ('') RETURNING material_id")
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let material_id: i32 = material.get("material_id");

    let dir_path = format!("./material{}", material_id);
    fs::create_dir_all(&dir_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut description_path: Option<String> = None;
    let mut attachments: Vec<AttachmentResponse> = Vec::new();

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or_default();
        let file_name = field.file_name().unwrap_or("file.dat").to_string();
        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        let save_path = format!("{}/{}", dir_path, file_name);
        fs::write(&save_path, &data).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if name == "description" && description_path.is_none() {
            description_path = Some(save_path.clone());

            sqlx::query("UPDATE materials SET description_path = $1 WHERE material_id = $2")
                .bind(&save_path)
                .bind(material_id)
                .execute(&pool)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        } else {
            sqlx::query(
                "INSERT INTO attachments (material_id, file_path, file_name) VALUES ($1, $2, $3)",
            )
            .bind(material_id)
            .bind(&save_path)
            .bind(&file_name)
            .execute(&pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            attachments.push(AttachmentResponse {
                file_name,
                file_path: save_path,
            });
        }
    }

    let Some(description_path) = description_path else {
        return Err(StatusCode::BAD_REQUEST);
    };

    let response = MaterialResponse {
        material_id,
        description_path,
        attachments,
    };

    Ok((StatusCode::CREATED, Json(response)))
}
