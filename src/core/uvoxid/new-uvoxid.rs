use crate::core::uvoxid::Delta;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::{Add, AddAssign};

const ANG_SCALE: i128 = 100_000_000_000;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RUm(pub i64);

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LatCode(pub i64);

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct LonCode(pub i64);

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UvoxId {
    pub r_um: RUm,
    pub lat_code: LatCode,
    pub lon_code: LonCode,
}

impl UvoxId {
    pub fn new(r_um: RUm, lat_code: LatCode, lon_code: LonCode) -> Self {
        Self {
            r_um: RUm(r_um.0.max(0)),
            lat_code,
            lon_code,
        }
    }

    pub fn earth(r_um: RUm, lat_code: LatCode, lon_code: LonCode) -> Self {
        Self::new(r_um, lat_code, lon_code)
    }

    pub fn as_tuple(&self) -> (RUm, LatCode, LonCode) {
        (self.r_um, self.lat_code, self.lon_code)
    }

    pub fn wrapping_add_lon(&mut self, delta: i64) {
        self.lon_code.0 = self.lon_code.0.wrapping_add(delta);
    }

    pub fn wrapping_add_lat(&mut self, delta: i64) {
        self.lat_code.0 = self.lat_code.0.wrapping_add(delta);
    }

    pub fn apply_delta(&mut self, delta: Delta) {
        self.r_um.0 = (self.r_um.0 as i128 + delta.dr_um as i128).max(0) as i64;

        let mut lat = self.lat_code.0 as i128 + delta.dlat as i128;
        let mut lon = self.lon_code.0 as i128 + delta.dlon as i128;

        // latitude wrap
        while lat > 90 * ANG_SCALE {
            lat = 180 * ANG_SCALE - lat;
            lon += 180 * ANG_SCALE;
        }
        while lat < -90 * ANG_SCALE {
            lat = -180 * ANG_SCALE - lat;
            lon += 180 * ANG_SCALE;
        }

        self.lat_code.0 = lat.clamp(-90 * ANG_SCALE, 90 * ANG_SCALE) as i64;

        // wrap lon to [-180°, 180°)
        self.lon_code.0 =
            ((lon + 180 * ANG_SCALE).rem_euclid(360 * ANG_SCALE) - 180 * ANG_SCALE) as i64;
    }

    pub fn to_hex(&self) -> String {
        format!(
            "{:016x}{:016x}{:016x}",
            self.r_um.0 as u64,
            self.lat_code.0 as u64,
            self.lon_code.0 as u64,
        )
    }

    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 48 { return None; }

        let r_bits   = u64::from_str_radix(&s[0..16], 16).ok()?;
        let lat_bits = u64::from_str_radix(&s[16..32], 16).ok()?;
        let lon_bits = u64::from_str_radix(&s[32..48], 16).ok()?;

        Some(Self {
            r_um: RUm(r_bits as i64),
            lat_code: LatCode(lat_bits as i64),
            lon_code: LonCode(lon_bits as i64),
        })
    }
}

impl Add<Delta> for UvoxId {
    type Output = UvoxId;

    fn add(mut self, delta: Delta) -> Self::Output {
        self.apply_delta(delta);
        self
    }
}

impl AddAssign<Delta> for UvoxId {
    fn add_assign(&mut self, delta: Delta) {
        self.apply_delta(delta);
    }
}

impl fmt::Display for UvoxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "r={}µm, lat={}, lon={}",
            self.r_um.0, self.lat_code.0, self.lon_code.0
        )
    }
}
