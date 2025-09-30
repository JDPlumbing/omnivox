use axum::Router;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

use omnivox_backend::api; // your API router

#[tokio::main]
async fn main() {
    // Build our application with routes and layers
    let app = Router::new()
        .nest("/api", api::api_router())
        .layer(CorsLayer::permissive()); // <- allows all origins (dev-friendly)

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}
