use axum::{extract::Extension, Json, http::StatusCode};
use sqlx::{PgPool, Row};
use bcrypt::{hash, DEFAULT_COST};
use crate::models::{User, CreateUser};
use crate::utils::validations::{validate_username, validate_password};

pub async fn create_user(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), StatusCode> {

    if !validate_username(&payload.username) {
        return Err(StatusCode::BAD_REQUEST);
    }

    if !validate_password(&payload.user_password) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let hashed_password = hash(payload.user_password, DEFAULT_COST)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let query = "
        INSERT INTO users (username, user_password, user_email, user_role) 
        VALUES ($1, $2, $3, $4) 
        RETURNING user_id, username, user_email, user_role
    ";

    let created_user = sqlx::query(query)
        .bind(payload.username)
        .bind(hashed_password.clone())
        .bind(payload.user_email)
        .bind(1)
        .fetch_one(&pool)
        .await;

    match created_user {
        Ok(created_user) => {
            let user_response = User {
                user_id: created_user.get("user_id"),
                username: created_user.get("username"),
                user_password: "{}".to_string(),
                user_email: created_user.get("user_email"),
                user_role: created_user.get("user_role"),
            };
            Ok((StatusCode::CREATED, Json(user_response)))
        }
        Err(sqlx::Error::Database(db_err)) if db_err.code().map_or(false, |code| code == "23505") => {
            Err(StatusCode::CONFLICT)
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use axum::{extract::Extension, Json};
    use std::env;
    use crate::models::CreateUser;

    #[tokio::test]
    async fn test_create_user_success_1() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }

    #[tokio::test]
    async fn test_create_user_success_2() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }

    #[tokio::test]
    async fn test_create_user_unsuccess_1() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }
    
    #[tokio::test]
    async fn test_create_user_unsuccess_2() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }

    #[tokio::test]
    async fn test_read_user_unsuccess_3() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }
    

    #[tokio::test]
    async fn test_read_user_unsuccess_4() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }

    #[tokio::test]
    async fn test_update_user_unsuccess_5() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_update_user_unsuccess_6() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }
    #[tokio::test]
    async fn test_delete_user_unsuccess_7() {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://dou_code_dba:Ahri34@@localhost:5432/dou_code")
            .await
            .expect("Error al conectar a la base de datos");

        let payload = CreateUser {
            username: "new_user123".to_string(),
            user_password: "PasswordFuerte123!".to_string(),
            user_email: "new_user@example.com".to_string(),
        };

        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let response = create_user(Extension(pool), Json(payload)).await;

        assert!(matches!(response, _response));
    }
}

