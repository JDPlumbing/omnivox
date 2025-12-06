use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::{Add, AddAssign};

use crate::core::uvoxid::{ANG_SCALE, RUm, LatCode, LonCode};

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
