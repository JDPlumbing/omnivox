use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::tdt::SimTime;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::frames::local_tangent_frame;
use crate::core::physics::camera::{
    pose::CameraPose,
    basis::camera_basis_from_enu,
};
use crate::core::physics::atmosphere::optics::{
    integrate_atmosphere_along_ray,
    AtmosphereOpticsParams,
};
use crate::core::env::atmosphere::AtmosphereField;
use crate::core::math::vec3::{ normalize, dot };
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::atmosphere::AtmosphereOpticsResponse;
use crate::core::id::WorldId;


#[derive(Deserialize)]
pub struct AtmosphereOpticsQuery {
    pub time_ns: Option<String>,
    pub yaw_deg: f64,
    pub pitch_deg: f64,
}

pub async fn atmosphere_optics_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<AtmosphereOpticsQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // ----------------------------
    // Observer
    // ----------------------------
    let observer = app.observers
        .read().await
        .get(&ObserverId(observer_id))
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    let time = q.time_ns
        .as_deref()
        .unwrap_or("0")
        .parse::<i128>()
        .map(SimTime)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // ----------------------------
    // World + environment
    // ----------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;
    let atmosphere_model = env.atmosphere
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let atmosphere = AtmosphereField::from_model(space, atmosphere_model);

    // ----------------------------
    // Resolver + local frame
    // ----------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let frame = local_tangent_frame(
        &resolver,
        observer.world,
        &observer.uvox,
        time,
        space,
    ).map_err(|_| StatusCode::BAD_REQUEST)?;

    // ----------------------------
    // Camera ray
    // ----------------------------
    let basis = camera_basis_from_enu(
        frame.enu,
        CameraPose {
            yaw_rad: q.yaw_deg.to_radians(),
            pitch_rad: q.pitch_deg.to_radians(),
        },
    );

    let ray_dir = normalize(basis.forward);
    // ----------------------------
    // Convert origin to Earth-centered space
    // ----------------------------
// Earth center (inertial)
let earth_center = resolver.world_pose(observer.world, time).position_m;

let observer_ec = [
    frame.origin[0] - earth_center[0],
    frame.origin[1] - earth_center[1],
    frame.origin[2] - earth_center[2],
];

// Observer world position
let observer_pos = frame.origin;

// Sun world position
let sun_pos = resolver.world_pose(WorldId(0), time).position_m;

// Sun direction FROM observer (world space)
let sun_dir_world = normalize([
    sun_pos[0] - observer_pos[0],
    sun_pos[1] - observer_pos[1],
    sun_pos[2] - observer_pos[2],
]);

// Local up direction (already world-space!)
let up_dir_world = frame.enu.up;

// Dot product gives sin(elevation)
let sun_dot_up = dot(sun_dir_world, up_dir_world).clamp(-1.0, 1.0);

let sun_elevation_deg = sun_dot_up.asin().to_degrees();
let sun_visibility = sun_dot_up.max(0.0);

println!(
    "sun elevation deg = {:.2}, visibility = {:.3}",
    sun_elevation_deg,
    sun_visibility
);

// Atmosphere integration
let result = integrate_atmosphere_along_ray(
    &atmosphere,
    observer_ec,        // Earth-centered origin (meters)
    ray_dir,            // camera ray (world space)
    sun_dir_world,      // Sun direction (world space)
    AtmosphereOpticsParams::default(),
);

// Derived perceptual output
let sky_luminance =
    result.sky_scatter_energy * sun_visibility * result.transmittance;

println!("sun elevation deg = {:.2}", sun_elevation_deg);


    Ok(Json(AtmosphereOpticsResponse {

    observer_id,
    time_ns: time.0,
    optical_depth: result.optical_depth,
    transmittance: result.transmittance,
    sun_visibility,
    sky_luminance,
    sky_scatter_energy: result.sky_scatter_energy,


    }))
}
