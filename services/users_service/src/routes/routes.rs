use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    create_users::create_user, 
    get_users::{get_users,get_user_by_id},
    update_users::update_user,
    delete_users::delete_user,
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_users) .post(create_user))
        .route("/{user_id}", get(get_user_by_id)) 
        .route("/{user_id}", put(update_user)) 
        .route("/{user_id}", delete(delete_user)) 

}




