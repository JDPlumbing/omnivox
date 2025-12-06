use serde::{Serialize, Deserialize};
use std::fmt;

use crate::core::id::uvoxid::{ANG_SCALE};
use crate::core::tdt::sim_duration::SimDuration;
use crate::core::uvoxid::Delta;
use crate::core::uvoxid::{DRUm, DLat, DLon};

/// -------------------------------
/// Velocity: units per second
/// -------------------------------
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Velocity {
    /// radial velocity in µm/s
    pub vr_um_s: f64,

    /// latitudinal angular velocity (degrees per second)
    pub vlat_deg_s: f64,

    /// longitudinal angular velocity (degrees per second)
    pub vlon_deg_s: f64,
}

impl Velocity {
    pub fn new(vr_um_s: f64, vlat_deg_s: f64, vlon_deg_s: f64) -> Self {
        Self {
            vr_um_s,
            vlat_deg_s,
            vlon_deg_s,
        }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Convert velocity * time → Delta
    ///
    /// dt is a SimDuration (ns), internally converted to seconds.
    pub fn to_delta(&self, dt: SimDuration) -> Delta {
        let dt_s = dt.as_seconds_f64();

        Delta::typed(
            DRUm((self.vr_um_s * dt_s) as i64),
            DLat((self.vlat_deg_s * ANG_SCALE as f64 * dt_s) as i64),
            DLon((self.vlon_deg_s * ANG_SCALE as f64 * dt_s) as i64),
        )
    }
}

impl fmt::Display for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Vr={} µm/s, Vlat={}°/s, Vlon={}°/s",
            self.vr_um_s, self.vlat_deg_s, self.vlon_deg_s
        )
    }
}
