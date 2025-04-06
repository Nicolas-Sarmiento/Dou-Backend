use axum::Router;
use axum::routing::post;
use crate::handlers::upload_code_handler::upload;

pub fn create_router() -> Router {
    Router::new()
        .route("/", post(upload))
}

