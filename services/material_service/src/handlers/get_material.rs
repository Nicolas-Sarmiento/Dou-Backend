use axum::{Extension, Json, http::StatusCode};
use sqlx::PgPool;
use crate::models::{MaterialResponse, AttachmentResponse};

pub async fn get_all_materials(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<MaterialResponse>>, StatusCode> {
    let materials = sqlx::query!(
        r#"SELECT material_id, description_path FROM materials"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut result = Vec::new();

    for mat in materials {
        let attachments = sqlx::query!(
            "SELECT file_name, file_path FROM attachments WHERE material_id = $1",
            mat.material_id
        )
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|row| AttachmentResponse {
            file_name: row.file_name,
            file_path: row.file_path,
        })
        .collect();

        result.push(MaterialResponse {
            material_id: mat.material_id,
            description_path: mat.description_path,
            attachments,
        });
    }

    Ok(Json(result))
}

pub async fn get_material_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<MaterialResponse>, StatusCode> {
    let mat = sqlx::query!(
        "SELECT material_id, description_path FROM materials WHERE material_id = $1",
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let Some(mat) = mat else {
        return Err(StatusCode::NOT_FOUND);
    };

    let attachments = sqlx::query!(
        "SELECT file_name, file_path FROM attachments WHERE material_id = $1",
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|row| AttachmentResponse {
        file_name: row.file_name,
        file_path: row.file_path,
    })
    .collect();

    Ok(Json(MaterialResponse {
        material_id: mat.material_id,
        description_path: mat.description_path,
        attachments,
    }))
}
