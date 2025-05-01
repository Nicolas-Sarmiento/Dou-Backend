use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    Json,
};
use sqlx::{PgPool, Row};
use tokio::fs::read_to_string;
use serde::Serialize;

#[derive(Serialize)]
pub struct Submission {
	submission_id: i32,
    user_id: i32,
    problem_id: i32,
    submission_content: String,
    submission_answer_code: String,
}

pub async fn get_submissions(
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<Vec<Submission>>), StatusCode> {
    let query = "
        SELECT
            submission_id,
            user_id,
            problem_id,
            submission_url,
            submission_answer_code
        FROM submissions
        ORDER BY submission_id ASC
    ";

    let rows = sqlx::query(query).fetch_all(&pool).await;

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


pub async fn get_submission_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>
) -> Result<(StatusCode, Json<Submission>), StatusCode> {
    let query = "
        SELECT
            submission_id,
            user_id,
            problem_id,
            submission_url,
            submission_answer_code
        FROM submissions
        WHERE submission_id = $1
        ORDER BY submission_id ASC
    ";

    let row = sqlx::query(query)
        .bind(id)
        .fetch_one(&pool)
        .await;

    match row {
        Ok(row) => {
            let submission_path: String = row.get("submission_url");

            let submisssion_content_file = match read_to_string(&submission_path).await {
                Ok(content) => content,
                Err(_) => String::from("[Error al leer el envío]"),
            };

            let submission = Submission {
                submission_id: row.get("submission_id"),
                user_id: row.get("user_id"),
                problem_id: row.get("problem_id"),
                submission_content: submisssion_content_file,
                submission_answer_code: row.get("submission_answer_code"),
            };
            

            Ok((StatusCode::OK, Json(submission)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_submissions_by_user_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>
) -> Result<(StatusCode, Json<Vec<Submission>>), StatusCode> {
    let query = "
        SELECT
            submission_id,
            user_id,
            problem_id,
            submission_url,
            submission_answer_code
        FROM submissions
        WHERE user_id = $1
        ORDER BY submission_id ASC
    ";

    let rows = sqlx::query(query).bind(id).fetch_all(&pool).await;

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






