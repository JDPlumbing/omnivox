use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};
use crate::shared::app_state::AppState;

// --- Submodules ---
pub mod runtime;
mod types;
mod create;
mod delete;
mod get;
mod list;
mod update;
mod init;
mod run;

// --- Public exports ---
pub use types::{SimulationDto, NewSimulation};
pub use create::create_simulation;
pub use delete::delete_simulation;
pub use get::get_simulation;
pub use list::list_simulations;
pub use update::{update_simulation, patch_simulation};
pub use init::init_simulation;
pub use run::run_simulation;

use runtime::{start_sim, tick_sim, stop_sim, list_sims};

/// Builds the simulation routes (used by `api/mod.rs`)
pub fn routes() -> Router<AppState> {
    Router::new()
        // Simulation DB routes
        .route("/", get(list_simulations).post(create_simulation))
        .route(
            "/{id}",
            get(get_simulation)
                .put(update_simulation)
                .patch(patch_simulation)
                .delete(delete_simulation),
        )
        .route("/init", post(init_simulation))
        .route("/run", post(run_simulation))
        // Runtime control routes
        .route("/runtime/start", post(start_sim))
        .route("/runtime/{id}/tick", post(tick_sim))
        .route("/runtime/{id}/stop", post(stop_sim))
        .route("/runtime/list", get(list_sims))
}
