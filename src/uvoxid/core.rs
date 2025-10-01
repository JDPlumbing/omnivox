use crate::uvoxid::Delta;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::{Add, AddAssign};

/// A UVoxID as 4x64-bit fields:
/// - frame_id: reference frame anchor (e.g. 0 = Earth, 1 = Moon, 2 = Sun, …)
/// - r_um: radial distance from frame center, in micrometers
/// - lat_code: latitude code (full 64-bit signed range)
/// - lon_code: longitude code (full 64-bit signed range)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UvoxId {
    pub frame_id: u64,
    pub r_um: u64,
    pub lat_code: i64,
    pub lon_code: i64,
}

impl UvoxId {
    /// Construct directly
    pub fn new(frame_id: u64, r_um: u64, lat_code: i64, lon_code: i64) -> Self {
        Self { frame_id, r_um, lat_code, lon_code }
    }

    /// Convenience for Earth (frame 0)
    pub fn earth(r_um: u64, lat_code: i64, lon_code: i64) -> Self {
        Self::new(0, r_um, lat_code, lon_code)
    }

    /// Return as tuple for math/serialization
    pub fn as_tuple(&self) -> (u64, u64, i64, i64) {
        (self.frame_id, self.r_um, self.lat_code, self.lon_code)
    }

    /// Wrap longitude safely
    pub fn wrapping_add_lon(&mut self, delta: i64) {
        self.lon_code = self.lon_code.wrapping_add(delta);
    }

    /// Wrap latitude safely
    pub fn wrapping_add_lat(&mut self, delta: i64) {
        self.lat_code = self.lat_code.wrapping_add(delta);
    }

    /// Apply a delta in µm/lat/lon codes (frame_id is fixed)
    pub fn apply_delta(&mut self, delta: Delta) {
        // radius can’t go negative
        self.r_um = (self.r_um as i128 + delta.dr_um as i128).max(0) as u64;

        // use i128 to avoid overflow when adding big deltas
        let mut lat = self.lat_code as i128 + delta.dlat as i128;
        let mut lon = self.lon_code as i128 + delta.dlon as i128;

        // loop in case the delta is huge (crossing poles multiple times)
        while lat > 90_000_000 {
            lat = 180_000_000 - lat;
            lon += 180_000_000;
        }
        while lat < -90_000_000 {
            lat = -180_000_000 - lat;
            lon += 180_000_000;
        }

        // clamp latitude into safe range
        self.lat_code = lat.clamp(-90_000_000, 90_000_000) as i64;

        // wrap longitude into [-180e6, 180e6)
        self.lon_code = ((lon + 180_000_000).rem_euclid(360_000_000) - 180_000_000) as i64;
    }


    /// Serialize to packed 256-bit hex string.
    pub fn to_hex(&self) -> String {
        format!(
            "{:016x}{:016x}{:016x}{:016x}",
            self.frame_id,
            self.r_um,
            self.lat_code as u64, // reinterpret signed as raw bits
            self.lon_code as u64,
        )
    }

    /// Parse from packed 256-bit hex string.
    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 64 {
            return None;
        }
        let frame_id = u64::from_str_radix(&s[0..16], 16).ok()?;
        let r_um     = u64::from_str_radix(&s[16..32], 16).ok()?;
        let lat_bits = u64::from_str_radix(&s[32..48], 16).ok()?;
        let lon_bits = u64::from_str_radix(&s[48..64], 16).ok()?;

        Some(Self {
            frame_id,
            r_um,
            lat_code: lat_bits as i64,
            lon_code: lon_bits as i64,
        })
    }
}



impl Add<Delta> for UvoxId {
    type Output = UvoxId;

    fn add(self, delta: Delta) -> Self::Output {
        let mut new = self;
        new.apply_delta(delta);
        new
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
            "frame={}, r={} µm, lat={}, lon={}",
            self.frame_id, self.r_um, self.lat_code, self.lon_code
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_hex() {
        let original = UvoxId::new(42, 1_000_000u64, 123_456i64, -654_321i64);

        let hex = original.to_hex();
        assert_eq!(hex.len(), 64, "hex string should always be 64 chars long");

        let decoded = UvoxId::from_hex(&hex).expect("should parse hex back");

        assert_eq!(decoded.frame_id, original.frame_id);
        assert_eq!(decoded.r_um, original.r_um);
        assert_eq!(decoded.lat_code, original.lat_code);
        assert_eq!(decoded.lon_code, original.lon_code);
    }
}
