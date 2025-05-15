use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::{
    create_material::create_material, 
    delete_material::delete_material, 
    get_materials::{get_materials, get_material_by_id, get_materials_only_id, search_materials_by_title},  
    update_material::update_material
};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(get_materials) .post(create_material))
        .route("/{material_id}", delete(delete_material) .get(get_material_by_id) .put(update_material))
        .route("/onlyid", get(get_materials_only_id))
        .route("/search", get(search_materials_by_title))
}
