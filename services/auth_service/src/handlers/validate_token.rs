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