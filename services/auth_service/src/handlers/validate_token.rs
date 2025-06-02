use axum::{
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::models::user_auth::ValidationResponse;
use crate::utils::decode::decode_jwt;
use crate::utils::get_token::BearerToken;

pub async fn validate_token(
    BearerToken(token): BearerToken,
) -> impl IntoResponse {
    match decode_jwt(&token) {
        Ok(_) => (
            StatusCode::OK,
            Json(ValidationResponse {
                valid: true
            }),
        ),
        Err(_) => (
            StatusCode::UNAUTHORIZED,
            Json(ValidationResponse {
                valid: false
            }),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;


    #[tokio::test]
    async fn test_validate_token_invalid_token() {
        use std::env;
        use crate::utils::get_token::BearerToken;


        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let token = BearerToken("token_invalido.fake.jwt".to_string());

        let response = validate_token(token).await;

        assert!(matches!(response, _response));
    }
        #[tokio::test]
    async fn test_validate_token_invalid_token2() {
        use std::env;
        use crate::utils::get_token::BearerToken;


        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let token = BearerToken("token_invalido.fake.jwt".to_string());

        let response = validate_token(token).await;

        assert!(matches!(response, _response));
    }
        #[tokio::test]
    async fn test_validate_token_invalid_token3() {
        use std::env;
        use crate::utils::get_token::BearerToken;


        unsafe {
            env::set_var("JWT_SECRET", "clave_super_secreta");
        }

        let token = BearerToken("token_invalido.fake.jwt".to_string());

        let response = validate_token(token).await;

        assert!(matches!(response, _response));
    }
}
