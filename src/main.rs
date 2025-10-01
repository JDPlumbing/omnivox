use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use omnivox::api::api_router; // 👈 from lib.rs re-export

#[tokio::main]
async fn main() {
     dotenvy::dotenv().ok();
    let app = Router::new()
        .nest("/api", api_router()) // 👈 mounts all /hello, /worlds, /users
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    println!("listening on http://localhost:8000");
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
