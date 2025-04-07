use axum::{
    extract::Extension,
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use tokio::fs::read_to_string;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AttemptRequest {
    user_id: i32,
    problem_id: i32,
}

#[derive(Serialize)]
pub struct Submission {
	submission_id: i32,
    user_id: i32,
    problem_id: i32,
    submission_content: String,
    submission_answer_code: String,
}

pub async fn get_attemps(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<AttemptRequest>,
) -> Result<(StatusCode, Json<Vec<Submission>>), StatusCode> {
    let query = "
        SELECT
            submission_id,
            user_id,
            problem_id,
            submission_url,
            submission_answer_code
        FROM submissions
        WHERE user_id = $1 AND problem_id = $2
        ORDER BY submission_id ASC
    ";

    let rows = sqlx::query(query)
        .bind(payload.user_id)
        .bind(payload.problem_id)
        .fetch_all(&pool).await;

    match rows {
        Ok(rows) => {
            let mut submissions = Vec::new();

            for row in rows {
                let submission_path: String = row.get("submission_url");

                let submisssion_content_file = match read_to_string(&submission_path).await {
                    Ok(content) => content,
                    Err(_) => String::from("[Error al leer el envío]"),
                };

                submissions.push(Submission {
                    submission_id: row.get("submission_id"),
                    user_id: row.get("user_id"),
                    problem_id: row.get("problem_id"),
                    submission_content: submisssion_content_file,
                    submission_answer_code: row.get("submission_answer_code"),
                });
            }

            Ok((StatusCode::OK, Json(submissions)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
