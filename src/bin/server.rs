use axum::{Router, Extension, middleware};
use tower_http::cors::{Any, CorsLayer};
use omnivox::api::api_router;
use omnivox::shared::app_state::AppState;
use tokio::net::TcpListener;
use omnivox::api::identity_middleware;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let app_state = AppState::from_env()?;

    let api = api_router(app_state.clone());

    let app = Router::new()
        // 1️⃣ middleware declared FIRST (runs second)
        .layer(middleware::from_fn(identity_middleware))
        // 2️⃣ Extension declared LAST (runs first)
        .layer(Extension(app_state.clone()))
        // 3️⃣ then routes
        .nest("/api", api)
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    println!("🚀 Listening on http://localhost:8000");

    axum::serve(listener, app).await?;
    Ok(())
}
