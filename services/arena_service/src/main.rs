use tokio::sync::Mutex;
use std::collections::VecDeque;
use std::net::SocketAddr;
use models::matchmaking::AppState;
use tokio::net::TcpListener;
use crate::routes::routes::create_router;

mod handlers;
mod database;
mod routes;
mod models;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db_pool = database::init_db().await.expect("Failed to connect to database");
    let app_state = AppState{
        db_pool,
        matchmaking_queue: Mutex::new(VecDeque::new()).into(),
        arenas: Mutex::new(std::collections::HashMap::new()).into()
    };

    let app = create_router().layer(axum::extract::Extension(app_state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let listener = TcpListener::bind(addr).await.expect("Failed to bind address");

    println!("Server running on {}", addr);

    axum::serve(listener, app.into_make_service())
        .await
        .expect("Server failed");
}
