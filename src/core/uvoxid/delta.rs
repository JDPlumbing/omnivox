use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::{Add, AddAssign};
use glam::Vec3;
use crate::core::uvoxid::{ANG_SCALE, RUm, LatCode, LonCode, UvoxId};

/// Typed Δr in micrometers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(transparent)]
pub struct DRUm(pub i64);

impl DRUm {
    #[inline]
    pub fn meters(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }
}

/// Typed Δlatitude
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(transparent)]
pub struct DLat(pub i64);

impl DLat {
    #[inline] pub fn degrees(&self) -> f64 { self.0 as f64 / ANG_SCALE as f64 }
    #[inline] pub fn radians(&self) -> f64 { self.degrees().to_radians() }
}

/// Typed Δlongitude
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(transparent)]
pub struct DLon(pub i64);

impl DLon {
    #[inline] pub fn degrees(&self) -> f64 { self.0 as f64 / ANG_SCALE as f64 }
    #[inline] pub fn radians(&self) -> f64 { self.degrees().to_radians() }
}

/// Final typed displacement vector
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Delta {
    pub dr: DRUm,
    pub dlat: DLat,
    pub dlon: DLon,
}

impl Delta {
    pub fn new(dr_um: i64, dlat: i64, dlon: i64) -> Self {
        Delta {
            dr: DRUm(dr_um),
            dlat: DLat(dlat),
            dlon: DLon(dlon),
        }
    }

    pub fn typed(dr: DRUm, dlat: DLat, dlon: DLon) -> Self {
        Delta { dr, dlat, dlon }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn scale(&self, k: f64) -> Self {
        Delta {
            dr: DRUm((self.dr.0 as f64 * k) as i64),
            dlat: DLat((self.dlat.0 as f64 * k) as i64),
            dlon: DLon((self.dlon.0 as f64 * k) as i64),
        }
    }
}

impl Add for Delta {
    type Output = Delta;

    fn add(self, rhs: Delta) -> Delta {
        Delta {
            dr: DRUm(self.dr.0 + rhs.dr.0),
            dlat: DLat(self.dlat.0 + rhs.dlat.0),
            dlon: DLon(self.dlon.0 + rhs.dlon.0),
        }
    }
}

impl AddAssign for Delta {
    fn add_assign(&mut self, rhs: Delta) {
        self.dr.0 += rhs.dr.0;
        self.dlat.0 += rhs.dlat.0;
        self.dlon.0 += rhs.dlon.0;
    }
}

impl fmt::Display for Delta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Δr={}µm, Δlat={}°, Δlon={}°",
            self.dr.0,
            self.dlat.degrees(),
            self.dlon.degrees()
        )
    }
}


impl Delta {
    /// Convert a global Cartesian movement vector (meters)
    /// into a ΔUvoxId in (dr, dlat, dlon).
    pub fn from_cartesian_move(pos: &UvoxId, v: Vec3) -> Delta {
        // Old position in Cartesian
        let (x, y, z) = pos.to_cartesian();
        let old = Vec3::new(x as f32, y as f32, z as f32);

        // New Cartesian = old + movement vector
        let new = old + v;

        // Convert Cartesian → spherical
        //
        // r = radius
        // lat = arcsin(z/r)
        // lon = atan2(y, x)
        //
        let r_new = new.length() as f64;

        let lat_new = (new.z as f64 / r_new).asin().to_degrees();
        let lon_new = (new.y as f64).atan2(new.x as f64).to_degrees();

        // Old spherical
        let old_r = pos.r_um.meters();
        let old_lat = pos.lat_code.degrees();
        let old_lon = pos.lon_code.degrees();

        // Compute deltas
        let dr_um = ((r_new - old_r) * 1_000_000.0) as i64;

        let dlat = ((lat_new - old_lat) * ANG_SCALE as f64) as i64;
        let dlon = ((lon_new - old_lon) * ANG_SCALE as f64) as i64;

        Delta {
            dr: DRUm(dr_um),
            dlat: DLat(dlat),
            dlon: DLon(dlon),
        }

    }
}
