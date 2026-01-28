use axum::{Router, middleware};
use tower_http::cors::{Any, CorsLayer};
use omnivox::api::{api_router, identity_middleware};
use omnivox::shared::app_state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let app_state = AppState::from_env()?;
    let api = api_router(app_state.clone());

    let app = Router::new()
        .nest("/api", api)
        .layer(
            middleware::from_fn_with_state(
                app_state,
                identity_middleware,
            )
        )
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
