use axum::{
    extract::{Multipart, Extension},
    http::StatusCode,
    Json,
    response::IntoResponse,
};
use sqlx::{PgPool, Row};
use tokio::fs;
use uuid::Uuid;
use std::path::Path;
use zip::ZipArchive;
use std::io::Cursor;
use serde_json::json;

use fs_extra::dir::copy as copy_dir;
use fs_extra::file::copy as copy_file;

use crate::models::models::{Problem};
use crate::utils::validations::{validate_limits, validate_test_cases_structure};

pub async fn create_problem(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let mut name = String::new();
    let mut t_limit = 0;
    let mut m_limit = 0;
    let mut zip_data = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        match field.name() {
            Some("name") => name = field.text().await.unwrap_or_default(),
            Some("t_limit") => {
                let text = field.text().await.unwrap_or_default();
                t_limit = text.parse().unwrap_or(0);
            }
            Some("m_limit") => {
                let text = field.text().await.unwrap_or_default();
                m_limit = text.parse().unwrap_or(0);
            }
            Some("zip") => {
                let bytes = field.bytes().await.unwrap_or_default();
                zip_data = Some(bytes);
            }
            _ => {}
        }
    }

    if name.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing required field: name"})),
        ));
    }

    if !validate_limits(m_limit, t_limit) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid memory or time limit"})),
        ));
    }

    let zip_bytes = zip_data.ok_or_else(|| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Missing required file: zip"})),
        )
    })?;

    let temp_id = Uuid::new_v4();
    let temp_path = format!("/tmp/{}", temp_id);

    fs::create_dir_all(&temp_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create temp directory: {}", e)})),
        )
    })?;

    let cursor = Cursor::new(&zip_bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Invalid zip file: {}", e)})),
        )
    })?;

    archive.extract(&temp_path).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to extract zip: {}", e)})),
        )
    })?;

    let statement_src = Path::new(&temp_path).join("statement");
    if !validate_test_cases_structure(&statement_src) {
        let _ = fs::remove_dir_all(&temp_path).await;
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid folder structure. Make sure it includes statement.txt, testCases/ and outputs/."})),
        ));
    }

    let problem_id = Uuid::new_v4();
    let problem_path = format!("/app/problems/{}", problem_id);
    let statement_dst = Path::new(&problem_path).join("statement");

    fs::create_dir_all(&statement_dst).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to create statement folder: {}", e)})),
        )
    })?;

    let options = fs_extra::dir::CopyOptions::new();

    copy_dir(&statement_src.join("testCases"), &statement_dst, &options).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to copy testCases: {}", e)})),
        )
    })?;

    copy_dir(&statement_src.join("outputs"), &statement_dst, &options).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to copy outputs: {}", e)})),
        )
    })?;

    copy_file(
        &statement_src.join("statement.txt"),
        &statement_dst.join("statement.txt"),
        &fs_extra::file::CopyOptions::new(),
    ).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("Failed to copy statement.txt: {}", e)})),
        )
    })?;

    let _ = fs::remove_dir_all(&temp_path).await;

    let statement_url = format!("{}/statement/statement.txt", problem_path);
    let test_cases_url = format!("{}/statement/testCases", problem_path);
    let outputs_url = format!("{}/statement/outputs", problem_path);

    let query = "
        INSERT INTO problems (
            PROBLEM_NAME,
            PROBLEM_STATEMENT_URL,
            PROBLEM_TEST_CASES_URL,
            PROBLEM_OUTPUTS_URL,
            PROBLEM_MEMORY_MB_LIMIT,
            PROBLEM_TIME_MS_LIMIT
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING
            PROBLEM_ID,
            PROBLEM_NAME,
            PROBLEM_STATEMENT_URL,
            PROBLEM_TEST_CASES_URL,
            PROBLEM_OUTPUTS_URL,
            PROBLEM_MEMORY_MB_LIMIT,
            PROBLEM_TIME_MS_LIMIT
    ";

    let result = sqlx::query(query)
        .bind(&name)
        .bind(&statement_url)
        .bind(&test_cases_url)
        .bind(&outputs_url)
        .bind(t_limit)
        .bind(m_limit)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(row) => {
            let response = Problem {
                problem_id: row.get("problem_id"),
                problem_name: row.get("problem_name"),
                problem_statement_url: row.get("problem_statement_url"),
                problem_test_cases_url: row.get("problem_test_cases_url"),
                problem_outputs_url: row.get("problem_outputs_url"),
                problem_memory_mb_limit: row.get("problem_memory_mb_limit"),
                problem_time_ms_limit: row.get("problem_time_ms_limit"),
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("Database error: {}", e) })),
        )),
    }
}
