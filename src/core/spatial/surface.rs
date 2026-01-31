use serde::{Serialize, Deserialize};
use crate::core::physics::units::angle::Radians;
use crate::core::physics::units::length::Meters;


/// A position relative to a world's surface.
/// This is NOT an address and has no identity.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SurfaceCoords {
    pub latitude: Radians,
    pub longitude: Radians,

    /// Elevation relative to the world's reference radius.
    /// Negative = subsurface, positive = above surface.
    pub elevation: Meters,
}


impl SurfaceCoords {
    pub fn on_surface(
        latitude: Radians,
        longitude: Radians,
    ) -> Self {
        Self {
            latitude,
            longitude,
            elevation: Meters(0.0),
        }
    }

    pub fn with_elevation(
        latitude: Radians,
        longitude: Radians,
        elevation: Meters,
    ) -> Self {
        Self {
            latitude,
            longitude,
            elevation,
        }
    }
}
