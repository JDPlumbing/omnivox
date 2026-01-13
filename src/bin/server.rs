#![cfg_attr(debug_assertions, allow(warnings))]

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use omnivox::api::api_router;
use omnivox::shared::app_state::AppState;
use tokio::net::TcpListener;
use axum::middleware;
use omnivox::shared::auth_middleware::populate_user_from_auth;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    // Build shared state ONCE
    let app_state = AppState::new_from_env()?;

    // Build the API router (it already has .with_state)
    let api = api_router(app_state);

    // Mount at /api
    let app = Router::new()
        .nest("/api", api)
        
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );


    // Start server
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    println!("🚀 Listening on http://localhost:8000");

    axum::serve(listener, app).await?;

    Ok(())
}
