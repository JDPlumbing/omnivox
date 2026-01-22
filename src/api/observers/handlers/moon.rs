use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::shared::app_state::AppState;
use crate::core::observer::ObserverId;
use crate::core::id::WorldId;
use crate::core::tdt::SimTime;
use crate::core::world::presets::frames::frame_presets;
use crate::core::world::world_frame::WorldResolver;
use crate::core::physics::frames::local_tangent_frame;
use crate::supabasic::worlds::WorldRow;

use crate::api::observers::dtos::moon::MoonAnglesResponse;
use crate::api::observers::dtos::moon::ObserverMoonPhaseResponse;
use crate::core::observer_lunar_phase_fraction;

// reuse the same query struct you use for sun/environment
#[derive(serde::Deserialize)]
pub struct TimeQuery {
    pub time_ns: Option<String>,
}

// local helpers (or import shared ones if you already have them)
fn magnitude(v: [f64; 3]) -> f64 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let m = magnitude(v).max(1e-12);
    [v[0] / m, v[1] / m, v[2] / m]
}

pub async fn moon_angles_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<TimeQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // -------------------------------------------------
    // Load observer
    // -------------------------------------------------
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

    // -------------------------------------------------
    // Load world + environment
    // -------------------------------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    // -------------------------------------------------
    // World resolver
    // -------------------------------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };

    let earth = observer.world;
    let moon = WorldId(2);

    // -------------------------------------------------
    // Observer and Moon positions
    // -------------------------------------------------
    let obs_pos = resolver
        .world_anchor_point(earth, &observer.uvox, time, space)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let moon_pos = resolver
        .world_pose(moon, time)
        .position_m;

    let v = [
        moon_pos[0] - obs_pos[0],
        moon_pos[1] - obs_pos[1],
        moon_pos[2] - obs_pos[2],
    ];

    let distance = magnitude(v);
    let dir = normalize(v);

    // -------------------------------------------------
    // Local tangent frame (ENU)
    // -------------------------------------------------
    let frame = local_tangent_frame(
        &resolver,
        earth,
        &observer.uvox,
        time,
        space,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let [e, n, u] = frame.enu.project(dir);

    let azimuth = e.atan2(n).to_degrees().rem_euclid(360.0);
    let elevation = u.asin().to_degrees();

    Ok(Json(MoonAnglesResponse {
        observer_id,
        time_ns: time.0,
        azimuth_deg: azimuth,
        elevation_deg: elevation,
        distance_m: distance,
    }))
}




pub async fn observer_moon_phase_handler(
    State(app): State<AppState>,
    Path(observer_id): Path<u64>,
    Query(q): Query<TimeQuery>,
) -> Result<impl IntoResponse, StatusCode> {
    // -------------------------------------------------
    // Load observer
    // -------------------------------------------------
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

    // -------------------------------------------------
    // Load world + environment
    // -------------------------------------------------
    let world_row = WorldRow::get(&app.supa, observer.world)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let env = world_row.environment
        .as_ref()
        .ok_or(StatusCode::BAD_REQUEST)?;

    let space = &env.space;

    // -------------------------------------------------
    // Resolver
    // -------------------------------------------------
    let frames = frame_presets();
    let resolver = WorldResolver { frames: &frames };
    
    // Canonical IDs
    let earth = observer.world;
    let moon = WorldId(2);
    let sun = WorldId(0);

    // -------------------------------------------------
    // Compute observer lunar phase
    // -------------------------------------------------
// -------------------------------------------------
// Compute observer lunar phase (angle + fraction)
// -------------------------------------------------

let moon_pos = resolver.world_pose(moon, time).position_m;
let sun_pos  = resolver.world_pose(sun, time).position_m;

let obs_pos = resolver.world_anchor_point(
    earth,
    &observer.uvox,
    time,
    space,
).map_err(|_| StatusCode::BAD_REQUEST)?;

// Direction vectors FROM moon
let to_sun = normalize([
    sun_pos[0] - moon_pos[0],
    sun_pos[1] - moon_pos[1],
    sun_pos[2] - moon_pos[2],
]);

let to_obs = normalize([
    obs_pos[0] - moon_pos[0],
    obs_pos[1] - moon_pos[1],
    obs_pos[2] - moon_pos[2],
]);

let cos_phase = (to_sun[0] * to_obs[0]
               + to_sun[1] * to_obs[1]
               + to_sun[2] * to_obs[2])
    .clamp(-1.0, 1.0);

let phase_angle_rad = cos_phase.acos();
let illuminated_fraction = 0.5 * (1.0 + cos_phase);


    Ok(Json(ObserverMoonPhaseResponse {
        observer_id,
        time_ns: time.0,
        illuminated_fraction,
        phase_angle_rad,
    }))
}