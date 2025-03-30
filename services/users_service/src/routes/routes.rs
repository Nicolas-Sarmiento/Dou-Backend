use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    create_users::create_user, 
    get_users::get_users,
    update_users::update_user,
    delete_users::delete_user,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/users", post(create_user).get(get_users))
        .route("/users/:user_id", put(update_user)) 
        .route("/users/:user_id", delete(delete_user)) 
}


