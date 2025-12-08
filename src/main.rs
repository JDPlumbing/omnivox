#![cfg_attr(debug_assertions, allow(warnings))]

use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use omnivox::api::api_router;
use omnivox::shared::app_state::AppState;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let app_state = AppState::new_from_env()?;   // Build state once

    // Build router with app state
    let app = Router::new()
        .nest("/api", api_router(app_state.clone()))
        .with_state(app_state)   // Attach global shared state
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // 🚀 START THE SERVER — THIS WAS MISSING
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    println!("🚀 Listening on http://localhost:8000");

    axum::serve(listener, app).await?;

    Ok(())
}

// KEEP YOUR TEST MODULE
#[cfg(test)]
mod tests {
    use super::*;
    use crate::matcat::materials::MatCatId;

    #[test]
    fn test_matcat_integrity() {
        let ids = vec![
            MatCatId::new(1, 1, 0),
            MatCatId::new(2, 1, 0),
            MatCatId::new(3, 0, 0),
            MatCatId::new(9, 0, 0),
        ];

        for id in ids {
            let name = id.name();
            let props = id.props().expect("should generate props");
            println!("🧱 {name}: {:?}", props);
        }
    }
}
