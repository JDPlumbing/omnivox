//! Energy computations: kinetic, potential, and deformation energy.

pub fn kinetic_energy(mass: f64, velocity: f64) -> f64 {
    0.5 * mass * velocity.powi(2)
}

pub fn potential_energy(mass: f64, height: f64, g: f64) -> f64 {
    mass * g * height
}

pub fn deformation_energy(force: f64, displacement: f64) -> f64 {
    0.5 * force * displacement
}
