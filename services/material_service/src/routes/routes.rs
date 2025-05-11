use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    create_material::create_material, delete_material::delete_material, get_material::{get_all_materials, get_material_by_id}   
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_all_materials) .post(create_material))
        .route("/{material_id}", delete(delete_material) .get(get_material_by_id))
}
