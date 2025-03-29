mod routes;


#[tokio::main]
async fn main() {
    let app = routes::routes::create_router();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Auth service running!");
    axum::serve(listener, app).await.unwrap();
}
