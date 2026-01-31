use crate::core::environment::conditions::EnvironmentConditions;
use crate::core::worlds::systems::gravity::LocalENU;

#[derive(Debug, Clone, Copy)]
pub struct EntityEnvironmentSample {
    /// Scalar, frame-independent environmental facts
    pub env: EnvironmentConditions,

    /// World-local gravitational field at this location (ENU)
    pub gravity_enu: LocalENU,
}
