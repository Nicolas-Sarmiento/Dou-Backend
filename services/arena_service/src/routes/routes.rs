use axum::Router;
use axum::routing::post;
use crate::handlers::auth_user::auth_user;

pub fn create_router() -> Router {
    Router::new()
        .route("/login", post(auth_user))
}

