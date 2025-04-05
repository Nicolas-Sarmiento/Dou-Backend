use axum::{
    extract::{Multipart, Extension},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use tokio::fs;
use uuid::Uuid;
use std::path::Path;
use zip::ZipArchive;
use std::fs::File;
use std::io::Cursor;
use crate::models::models::{CreateProblem, Problem};
use crate::utils::validations::{validate_limits, validate_test_cases_structure};


pub async fn create_problem(
    Extension(pool): Extension<PgPool>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<Problem>), StatusCode> {
    let mut name = String::new();
    let mut t_limit = 0;
    let mut m_limit = 0;
    let mut zip_data = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        match field.name() {
            Some("name") => {
                name = field.text().await.unwrap_or_default();
            }
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
            Some(name) => {
            }
            None => {
            }
        }
    }

    if !validate_limits(m_limit, t_limit) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let problem_id = Uuid::new_v4();
    let problem_path = format!("/app/problems/{}", problem_id);

    fs::create_dir_all(&problem_path)
        .await
        .map_err(|e| {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let zip_bytes = zip_data.ok_or_else(|| {
        StatusCode::BAD_REQUEST
    })?;

    let zip_file_path = format!("{}/statement.zip", problem_path);

    fs::write(&zip_file_path, &zip_bytes)
        .await
        .map_err(|e| {
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let zip_file = File::open(&zip_file_path).map_err(|e| {
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let mut archive = ZipArchive::new(zip_file).map_err(|e| {
        StatusCode::BAD_REQUEST
    })?;
    archive.extract(&problem_path).map_err(|e| {
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let statement_folder = Path::new(&problem_path).join("statement");

    if !validate_test_cases_structure(&statement_folder) {
        return Err(StatusCode::BAD_REQUEST);
    }

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
                problem_id: row.try_get("PROBLEM_ID").unwrap_or_default(),
                problem_name: row.get("problem_name"),
                problem_statement_url: row.get("problem_statement_url"),
                problem_test_cases_url: row.get("problem_test_cases_url"),
                problem_outputs_url: row.get("problem_outputs_url"),
                problem_memory_mb_limit: row.get("problem_memory_mb_limit"),
                problem_time_ms_limit: row.get("problem_time_ms_limit"),
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}