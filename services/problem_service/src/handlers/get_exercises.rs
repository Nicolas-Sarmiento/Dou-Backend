use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use crate::models::models::Problem;

pub async fn get_problems(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<Problem>>), StatusCode> {
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

    let rows = sqlx::query(query)
        .fetch_all(&pool)
        .await;

    match rows {
        Ok(rows) => {
            let problems: Vec<Problem> = rows
                .into_iter()
                .map(|row| Problem {
                    problem_id: row.get("problem_id"),
                    problem_name: row.get("problem_name"),
                    problem_statement_url: row.get("problem_statement_url"),
                    problem_test_cases_url: row.get("problem_test_cases_url"),
                    problem_outputs_url: row.get("problem_outputs_url"),
                    problem_memory_mb_limit: row.get("problem_memory_mb_limit"),
                    problem_time_ms_limit: row.get("problem_time_ms_limit"),
                })
                .collect();

            Ok((StatusCode::OK, Json(problems)))
        }
        Err(e) => {
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
