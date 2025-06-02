use axum::{
    extract::{Extension,Path},
    http::StatusCode,
    Json,
};

use sqlx::{PgPool, Row};
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
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_problems_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>
) -> Result<(StatusCode, Json<ProblemWithStatement>), StatusCode> {
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
        WHERE problem_id = $1;
    ";

    let row= sqlx::query(query)
        .bind(id)
        .fetch_one(&pool).await;

    match row {
        Ok(row) => {
            let statement_path: String = row.get("problem_statement_url");
            let statement_content = match read_to_string(&statement_path).await {
                Ok(content) => content,
                Err(_) => String::from("[Error al leer el statement.txt]"),
            };

            let problem = ProblemWithStatement { 
                problem_id: row.get("problem_id"),
                problem_name: row.get("problem_name"),
                problem_statement: statement_content,
                problem_test_cases_url: row.get("problem_test_cases_url"),
                problem_outputs_url: row.get("problem_outputs_url"),
                problem_memory_mb_limit: row.get("problem_memory_mb_limit"),
                problem_time_ms_limit: row.get("problem_time_ms_limit"),
            };
            Ok((StatusCode::OK, Json(problem)))
        }
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use axum::{extract::{Extension, Path}, Json};
    use sqlx::postgres::PgPoolOptions;
    use std::env;



    #[tokio::test]
    async fn test_get_problems_by_id_1() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
        
    #[tokio::test]
    async fn test_get_problems_by_id_2() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_get_problems_by_id_3() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_create_problem_1() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_create_problem_2() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_create_problem_3() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_create_problem_4() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_delete_problem_1() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_delete_problem_2() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_delete_problem_3() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = get_problems_by_id(Extension(pool), Path(1)).await;

        assert!(matches!(response, _response));
    }
}
