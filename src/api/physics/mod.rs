use axum::{
    routing::{get, post, put, patch, delete},
    Router,
};

pub mod handlers;
pub use handlers::*;



    pub fn physics_routes() -> Router {
        Router::new()
        .route("/lunar/phase", get(lunar_phase_handler))
        .route("/illumination", get(solar_illumination_handler))
        .route("/seasons/check", get(seasons_check_handler))
        .route("/tides", get(tidal_potential_handler))
        .route("/tides/acceleration", get(tidal_acceleration_handler))
        .route("/tides/curve", get(tides_curve_handler))
        .route("/insolation/daily", get(daily_insolation_handler))
        .route("/insolation/curve", get(insolation_curve_handler))
        .route("/insolation/seasons", get(insolation_seasons_handler))
        .route("/environmental_snapshot", get(environmental_snapshot_handler))
        .route("/environmental_snapshot/curve", get(environmental_snapshot_curve_handler))
        
        
        
        ;
    }