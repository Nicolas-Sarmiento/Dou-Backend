use axum::Router;
use axum::routing::{post, get};
use crate::handlers::auth_user::auth_user;
use crate::handlers::validate_token::validate_token;

pub fn create_router() -> Router {
    Router::new()
        .route("/login", post(auth_user))
        .route("/validate",get(validate_token))
}

