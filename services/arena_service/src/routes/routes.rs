use axum::Router;
use axum::routing::get;
use crate::handlers::matchmaking::matchmaking;

pub fn create_router() -> Router {
    Router::new()
        .route("/look_match", get(matchmaking))
}

