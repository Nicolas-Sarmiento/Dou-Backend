use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    create_exercise::create_problem, delete_exercise::delete_problem, get_exercises::{get_problems, get_problems_by_id}   
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_problems) .post(create_problem))
        .route("/{problem_id}", delete(delete_problem) .get(get_problems_by_id))
}
