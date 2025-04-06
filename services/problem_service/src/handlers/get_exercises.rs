use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use crate::models::models::Problem;
use tokio::fs::read_to_string;
use serde::Serialize;

#[derive(Serialize)]
pub struct ProblemWithStatement {
    problem_id: i32,
    problem_name: String,
    problem_statement: String, // <- Aquí va el contenido
    problem_test_cases_url: String,
    problem_outputs_url: String,
    problem_memory_mb_limit: i32,
    problem_time_ms_limit: i32,
}

pub async fn get_problems(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<ProblemWithStatement>>), StatusCode> {
    let query = "
        SELECT
            problem_id,
            problem_name,
            problem_statement_url,
            problem_test_cases_url,
            problem_outputs_url,
            problem_memory_mb_limit,
            problem_time_ms_limit
        FROM problems
        ORDER BY problem_id ASC
    ";

    let rows = sqlx::query(query).fetch_all(&pool).await;

    match rows {
        Ok(rows) => {
            let mut problems = Vec::new();

            for row in rows {
                let statement_path: String = row.get("problem_statement_url");

                let statement_content = match read_to_string(&statement_path).await {
                    Ok(content) => content,
                    Err(_) => String::from("[Error al leer el statement.txt]"),
                };

                problems.push(ProblemWithStatement {
                    problem_id: row.get("problem_id"),
                    problem_name: row.get("problem_name"),
                    problem_statement: statement_content,
                    problem_test_cases_url: row.get("problem_test_cases_url"),
                    problem_outputs_url: row.get("problem_outputs_url"),
                    problem_memory_mb_limit: row.get("problem_memory_mb_limit"),
                    problem_time_ms_limit: row.get("problem_time_ms_limit"),
                });
            }

            Ok((StatusCode::OK, Json(problems)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
