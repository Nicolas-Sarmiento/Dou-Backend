use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    create_exercise::create_problem,
    get_exercises::get_problems,
    delete_exercise::delete_problem,   
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_problems) .post(create_problem))
        .route("/{problem_id}", delete(delete_problem))
}
