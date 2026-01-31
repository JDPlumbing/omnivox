use crate::core::physics::units::length::Meters;

#[derive(Debug, Clone, Copy)]
pub enum WorldSurface {
    /// Perfect sphere (planetary body)
    Spherical {
        radius: Meters,
    },

    /// Infinite plane (space station, test worlds)
    Plane {
        elevation: Meters,
    },
}

impl Default for WorldSurface {
    fn default() -> Self {
        WorldSurface::Spherical {
            radius: Meters(1_000_000.0),
        }
    }
}
