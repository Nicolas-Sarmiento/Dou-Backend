use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::create_exercise::create_problem;

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(|| async { "HELLO WORLD" }).post(create_problem))
}
