use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::routes::create_router;

mod handlers;
mod database;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_pool = database::init_db().await.expect("Failed to connect to database");
    
    let app = create_router().layer(axum::extract::Extension(db_pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    println!("Server running on {}", addr);

    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");

    println!("Server running on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}
