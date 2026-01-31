use crate::core::cosmic::state::CosmicState;
use crate::core::cosmic::id::CosmicBodyId;
use crate::core::physics::constants::universal::G;
use crate::core::physics::units::acceleration::MetersPerSecondSquared;
use crate::core::math::vec3::Vec3;

use super::gravity_math::GravitationalAcceleration;

pub fn acceleration_vector_at_point_from_body(
    state: &CosmicState,
    body: CosmicBodyId,
    point_position: Vec3,
    body_position: Vec3,
) -> Vec3 {
    let mass = state.masses[&body].kg.0;

    let r_vec = body_position - point_position;
    let r = r_vec.magnitude().max(1.0);
    let direction = r_vec.normalized();

    let a_mag = G * mass / (r * r);

    direction * a_mag
}

pub fn gravitational_acceleration_at_point_from_body(
    state: &CosmicState,
    body: CosmicBodyId,
    point_position: Vec3,
    body_position: Vec3,
) -> GravitationalAcceleration {
    let r_vec = body_position - point_position;
    let direction = r_vec.normalized();

    let magnitude = MetersPerSecondSquared(
        acceleration_vector_at_point_from_body(
            state,
            body,
            point_position,
            body_position,
        ).magnitude()
    );

    GravitationalAcceleration {
        direction,
        magnitude,
    }
}
