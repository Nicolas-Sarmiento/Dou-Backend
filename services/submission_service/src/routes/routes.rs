use axum::Router;
use axum::routing::{post,get};
use crate::handlers::upload_code_handler::upload;
use crate::handlers::get_submissions_handler::{get_submissions, get_submission_by_id, get_submission_by_user_id};
use crate::handlers::get_attemps_handler::get_attemps;

pub fn create_router() -> Router {
    Router::new()
        .route("/", post(upload))
        .route("/", get(get_submissions))
        .route("/{id}", get(get_submission_by_id))
        .route("/attemps", post(get_attemps))
        .route("/user/{id}", get(get_submission_by_user_id))
}

