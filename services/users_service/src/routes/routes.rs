use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::create_users::create_user;

pub fn create_router() -> Router {
    Router::new()
        .route("/users", post(create_user))
}


