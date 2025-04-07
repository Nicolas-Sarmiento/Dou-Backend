use axum::Router;
use axum::routing::{post,get};
use crate::handlers::upload_code_handler::upload;
use crate::handlers::get_submissions_handler::{get_submissions, get_submission_by_id};

pub fn create_router() -> Router {
    Router::new()
        .route("/", post(upload))
        .route("/", get(get_submissions))
        .route("/{id}", get(get_submission_by_id))
}

