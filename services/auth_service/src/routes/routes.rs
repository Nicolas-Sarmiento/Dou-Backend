use axum::Router;
use axum::routing::post;
use crate::handlers::auth_user::auth_user;

pub fn create_router() -> Router {
    Router::new()
        .route("/auth_user", post(auth_user))
}

