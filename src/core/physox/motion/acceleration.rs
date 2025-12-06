use serde::{Serialize, Deserialize};
use std::fmt;

use crate::core::tdt::sim_duration::SimDuration;
use crate::core::uvoxid::Delta;
use crate::core::uvoxid::{DRUm, DLat, DLon};
use crate::core::id::uvoxid::ANG_SCALE;

/// -------------------------------
/// Acceleration: units per second²
/// -------------------------------
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Acceleration {
    /// radial acceleration in µm/s²
    pub ar_um_s2: f64,

    /// latitudinal angular acceleration (°/s²)
    pub alat_deg_s2: f64,

    /// longitudinal angular acceleration (°/s²)
    pub alon_deg_s2: f64,
}

impl Acceleration {
    pub fn new(ar_um_s2: f64, alat_deg_s2: f64, alon_deg_s2: f64) -> Self {
        Self { ar_um_s2, alat_deg_s2, alon_deg_s2 }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Convert accel * dt² → Delta
    ///
    /// Uses classical: Δx = ½ a t²
    pub fn to_delta(&self, dt: SimDuration) -> Delta {
        let dt_s = dt.as_seconds_f64();
        let half_t2 = 0.5 * dt_s * dt_s;

        Delta::typed(
            DRUm((self.ar_um_s2 * half_t2) as i64),
            DLat((self.alat_deg_s2 * ANG_SCALE as f64 * half_t2) as i64),
            DLon((self.alon_deg_s2 * ANG_SCALE as f64 * half_t2) as i64),
        )
    }
}

impl fmt::Display for Acceleration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ar={} µm/s², Alat={}°/s², Alon={}°/s²",
            self.ar_um_s2, self.alat_deg_s2, self.alon_deg_s2
        )
    }
}
