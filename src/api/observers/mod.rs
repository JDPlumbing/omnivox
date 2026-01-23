

use axum::{Router, routing::{get, post}};
use crate::shared::app_state::AppState;

mod handlers;
mod dtos;

pub fn observer_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_observer))
        .route("/{id}", get(handlers::get_observer))
        .route("/{id}/environment", get(handlers::sample_observer_environment))
        .route("/{id}/environment/curve", get(handlers::environmental_curve_handler))
        .route("/{id}/frame", get(handlers::get_observer_frame))
        .route("/{id}/sun/angles", get(handlers::get_observer_sun_angles))
        .route("/{id}/moon/angles", get(handlers::moon_angles_handler))
        .route("/{id}/moon/phase", get(handlers::observer_moon_phase_handler))
        .route("/{id}/camera/sky", get(handlers::camera_sky_handler))
        .route("/{id}/camera/sky/projected", get(handlers::camera_sky_projected_handler))
      
        .route("/{id}/camera/horizon", get(handlers::camera_horizon_handler))
        .route("/{id}/camera/eclipse", get(handlers::camera_eclipse_handler))
        .route("/{id}/camera/eclipse/timeline", get(handlers::camera_eclipse_timeline_handler))
        .route("/{id}/atmosphere/optics",get(handlers::atmosphere_optics_handler))
        .route("/{id}/atmosphere/sample", get(handlers::atmosphere_sample_handler))
        .route("/{id}/atmosphere/sweep", get(handlers::atmosphere_sweep_handler))
        .route("/{id}/pressure/sample", get(handlers::pressure_sample_handler))
        .route("/{id}/pressure/sweep", get(handlers::pressure_sweep_handler))
        .route("/{id}/chemistry/atmosphere", get(handlers::chemistry_atmosphere::chemistry_atmosphere_handler))
        .route("/{id}/chemistry/ocean", get(handlers::chemistry_ocean::chemistry_ocean_handler))



        .route("/{id}/surface/energy", get(handlers::surface_energy_handler))

}
