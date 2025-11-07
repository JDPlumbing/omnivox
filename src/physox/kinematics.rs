//! Basic motion integration and vector helpers.

pub fn integrate_position(pos: f64, vel: f64, dt: f64) -> f64 {
    pos + vel * dt
}

pub fn integrate_velocity(vel: f64, acc: f64, dt: f64) -> f64 {
    vel + acc * dt
}

/// Computes displacement under constant acceleration.
pub fn displacement(v0: f64, acc: f64, dt: f64) -> f64 {
    v0 * dt + 0.5 * acc * dt * dt
}
