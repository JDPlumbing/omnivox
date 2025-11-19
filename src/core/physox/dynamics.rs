//! Force, impulse, and momentum relations (Newtonian mechanics).

/// F = m * a → returns acceleration (a)
pub fn apply_force(force: f64, mass: f64) -> f64 {
    if mass == 0.0 { 0.0 } else { force / mass }
}

/// Δv = J / m → change in velocity from impulse
pub fn impulse_change(impulse: f64, mass: f64) -> f64 {
    if mass == 0.0 { 0.0 } else { impulse / mass }
}

/// F = Δp / Δt → impulse from force over time
pub fn impulse(force: f64, dt: f64) -> f64 {
    force * dt
}
