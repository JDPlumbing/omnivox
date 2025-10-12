use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use omnivox::api::api_router;
use omnivox::shared::app_state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let app_state = AppState::new_from_env()?;

    let app = Router::new()
        .nest("/api", api_router())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state.clone()); // 👈 attach state this way now

    let listener = TcpListener::bind("0.0.0.0:8000").await?;

    println!("🚀 Listening on http://localhost:8000");

    axum::serve(listener, app).await?;

    Ok(())
}
