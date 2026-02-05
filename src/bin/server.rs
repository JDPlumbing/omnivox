use std::sync::{Arc, Mutex};

use axum::Router;
use tokio::net::TcpListener;

use omnivox::core::simulation::sim_engine::SimulationEngine;
use omnivox::core::tdt::sim_time::SimTime;
use omnivox::shared::entities::entity_store::EntityStore;
use omnivox::core::cosmic::state::CosmicState;
use omnivox::core::worlds::state::WorldState;
use omnivox::core::environment::state::EnvironmentState;
use omnivox::shared::app_state::AppState;
use omnivox::api::api_router::api_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // -------------------------------------------------
    // Build core simulation state (once)
    // -------------------------------------------------
    let cosmic = CosmicState::demo_solar_system();
    let world = WorldState::demo_worlds();
    let environment = EnvironmentState::demo_earth();

    let engine = SimulationEngine::new_with_full_state(
        SimTime::from_seconds_f64(0.0),
        3_600_000_000_000, // 24 hours per tick
        cosmic,
        world,
        environment,
        EntityStore::default(),
    );

    // -------------------------------------------------
    // Wrap engine for shared access
    // -------------------------------------------------
    let engine = Arc::new(Mutex::new(engine));

    // -------------------------------------------------
    // Background ticker (time passes)
    // -------------------------------------------------
    let engine_clone = engine.clone();
    tokio::spawn(async move {
        let mut interval =
            tokio::time::interval(std::time::Duration::from_millis(16));

        loop {
            interval.tick().await;
            let mut engine = engine_clone.lock().unwrap();
            
            engine.tick();
        }
    });

    // -------------------------------------------------
    // Build app state
    // -------------------------------------------------
    let app_state = AppState { engine };

    // -------------------------------------------------
    // Build API
    // -------------------------------------------------
    let app = Router::new()
        .nest("/api", api_router(app_state));

    // -------------------------------------------------
    // Serve
    // -------------------------------------------------
    let listener = TcpListener::bind("0.0.0.0:8000").await?;
    println!("🚀 Listening on http://localhost:8000");

    axum::serve(listener, app).await?;
    Ok(())
}
