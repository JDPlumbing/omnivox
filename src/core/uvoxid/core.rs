use crate::core::uvoxid::Delta;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::ops::{Add, AddAssign};

/// Angular scale: 1e11 units per degree.
/// (Your original representation — keep this stable.)
pub const ANG_SCALE: i128 = 100_000_000_000;

/// Earth constants (μm)
pub const EARTH_RADIUS_UM: i64 = 6_371_000_000_000;
pub const ATMOSPHERE_LOWER_UM: i64 = 100_000_000_000; // 100 km
pub const ATMOSPHERE_UPPER_UM: i64 = 500_000_000_000; // NEAR SPACE

// ------------------------------------------------------------
// Typed coordinate wrappers
// ------------------------------------------------------------

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd,)]
#[repr(transparent)]
pub struct RUm(pub i64);

impl RUm {
    #[inline]
    pub fn meters(&self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    /// Which broad “environment layer” is this radius in?
    pub fn layer(&self) -> EnvironmentLayer {
        match self.0 {
            r if r < EARTH_RADIUS_UM => EnvironmentLayer::Subsurface,
            r if r < EARTH_RADIUS_UM + ATMOSPHERE_LOWER_UM => EnvironmentLayer::AtmosphereLow,
            r if r < EARTH_RADIUS_UM + ATMOSPHERE_UPPER_UM => EnvironmentLayer::AtmosphereHigh,
            _ => EnvironmentLayer::Vacuum,
        }
    }
}

impl fmt::Display for RUm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::AddAssign<i64> for RUm {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs;
    }
}

impl std::ops::Sub for RUm {
    type Output = i64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0 - rhs.0
    }
}
impl std::ops::Sub<i64> for RUm {
    type Output = i64;

    fn sub(self, rhs: i64) -> Self::Output {
        self.0 - rhs
    }
}


#[derive(Debug)]
pub enum EnvironmentLayer {
    Subsurface,
    AtmosphereLow,
    AtmosphereHigh,
    Vacuum,
}

// ------------------------------------------------------------
// Latitude wrapper
// ------------------------------------------------------------

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd)]
#[repr(transparent)]
pub struct LatCode(pub i64);

impl LatCode {
    #[inline]
    pub fn degrees(&self) -> f64 {
        (self.0 as f64) / (ANG_SCALE as f64)
    }

    #[inline]
    pub fn radians(&self) -> f64 {
        self.degrees().to_radians()
    }

    pub fn hemisphere(&self) -> Hemisphere {
        match self.0.cmp(&0) {
            std::cmp::Ordering::Greater => Hemisphere::North,
            std::cmp::Ordering::Less => Hemisphere::South,
            _ => Hemisphere::Equator,
        }
    }

    pub fn is_tropic(&self) -> bool {
        self.degrees().abs() <= 23.5
    }

    pub fn clamp_valid(&mut self) {
        let max = (90 * ANG_SCALE) as i64;
        let min = (-90 * ANG_SCALE) as i64;
        self.0 = self.0.clamp(min, max);

    }
}
impl fmt::Display for LatCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.degrees())
    }
}

impl std::ops::AddAssign<i64> for LatCode {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs;
    }
}

impl LatCode {
    pub fn from_degrees(deg: f64) -> Self {
        LatCode((deg * ANG_SCALE as f64) as i64)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hemisphere {
    North,
    South,
    Equator,
}

// ------------------------------------------------------------
// Longitude wrapper
// ------------------------------------------------------------

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd)]
#[repr(transparent)]
pub struct LonCode(pub i64);

impl LonCode {
    #[inline]
    pub fn degrees(&self) -> f64 {
        (self.0 as f64) / (ANG_SCALE as f64)
    }

    #[inline]
    pub fn radians(&self) -> f64 {
        self.degrees().to_radians()
    }

    /// Normalize to [-180°, 180°)
    pub fn wrap(&mut self) {
        let full: i128 = 360 * ANG_SCALE;
        let half: i128 = 180 * ANG_SCALE;

        let v = self.0 as i128;

        let wrapped = ((v + half).rem_euclid(full) - half) as i64;

        self.0 = wrapped;
    }

}
impl LonCode {
    pub fn from_degrees(deg: f64) -> Self {
        LonCode((deg * ANG_SCALE as f64) as i64)
    }
}

impl fmt::Display for LonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.degrees())
    }
}
impl std::ops::AddAssign<i64> for LonCode {
    fn add_assign(&mut self, rhs: i64) {
        self.0 += rhs;
    }
}
// ------------------------------------------------------------
// The Fully Typed UvoxId
// ------------------------------------------------------------

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UvoxId {
    pub r_um: RUm,
    pub lat_code: LatCode,
    pub lon_code: LonCode,
}

impl UvoxId {
    pub fn new(r_um: RUm, lat_code: LatCode, lon_code: LonCode) -> Self {
        let mut id = Self { r_um, lat_code, lon_code };
        id.lat_code.clamp_valid();
        id.lon_code.wrap();
        id
    }

    /// Convenience: create an Earth-surface point
    pub fn earth_surface(lat: LatCode, lon: LonCode) -> Self {
        Self::new(RUm(EARTH_RADIUS_UM), lat, lon)
    }

    pub fn as_tuple(&self) -> (RUm, LatCode, LonCode) {
        (self.r_um, self.lat_code, self.lon_code)
    }

    /// Convert to 3D Cartesian (meters)
    pub fn to_cartesian(&self) -> (f64, f64, f64) {
        let r = self.r_um.meters();
        let lat = self.lat_code.radians();
        let lon = self.lon_code.radians();

        let x = r * lat.cos() * lon.cos();
        let y = r * lat.cos() * lon.sin();
        let z = r * lat.sin();

        (x, y, z)
    }

    /// Apply movement delta (radial + angular)
    pub fn apply_delta(&mut self, delta: Delta) {
        // ---- Radial
    self.r_um.0 = (self.r_um.0 as i128 + delta.dr.0 as i128).max(0) as i64;



        // ---- Angular
        let mut lat = self.lat_code.0 as i128 + delta.dlat.0 as i128;
        let mut lon = self.lon_code.0 as i128 + delta.dlon.0 as i128;

        // Latitude reflection
        while lat > 90 * ANG_SCALE {
            lat = 180 * ANG_SCALE - lat;
            lon += 180 * ANG_SCALE;
        }
        while lat < -90 * ANG_SCALE {
            lat = -180 * ANG_SCALE - lat;
            lon += 180 * ANG_SCALE;
        }

        self.lat_code.0 = lat.clamp(-90 * ANG_SCALE, 90 * ANG_SCALE) as i64;

        // Longitude wrap
        self.lon_code.0 = ((lon + 180 * ANG_SCALE)
            .rem_euclid(360 * ANG_SCALE)
            - 180 * ANG_SCALE) as i64;
    }

    // --------------------------------------------------------
    // Hex packing for DB / URLs (unchanged behavior)
    // --------------------------------------------------------
    pub fn to_hex(&self) -> String {
        format!(
            "{:016x}{:016x}{:016x}",
            self.r_um.0 as u64,
            self.lat_code.0 as u64,
            self.lon_code.0 as u64,
        )
    }

    pub fn from_hex(s: &str) -> Option<Self> {
        if s.len() != 48 {
            return None;
        }
        let r = u64::from_str_radix(&s[0..16], 16).ok()? as i64;
        let lat = u64::from_str_radix(&s[16..32], 16).ok()? as i64;
        let lon = u64::from_str_radix(&s[32..48], 16).ok()? as i64;

        Some(Self::new(RUm(r), LatCode(lat), LonCode(lon)))
    }
    
    // ----------------------------------------------------------
    // Camera Conversion
    // ----------------------------------------------------------
    pub fn relative_to_camera(entity: &UvoxId, camera: &UvoxId) -> (f64, f64, f64) {
        let (ex, ey, ez) = entity.to_cartesian();
        let (cx, cy, cz) = camera.to_cartesian();
        (ex - cx, ey - cy, ez - cz)
    }

}
use std::str::FromStr;

impl FromStr for UvoxId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 1. Try hex
        if let Some(id) = UvoxId::from_hex(s) {
            return Ok(id);
        }

        // 2. Try other syntaxes later
        // if let Some(id) = parse_lat_lon(s) { ... }

        Err(format!("Invalid UvoxId string: {}", s))
    }
}

// ------------------------------------------------------------
// Operator overloads for delta math
// ------------------------------------------------------------

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

// ------------------------------------------------------------
// Display
// ------------------------------------------------------------

impl fmt::Display for UvoxId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "r={}µm, lat={}°, lon={}°",
            self.r_um.0,
            self.lat_code.degrees(),
            self.lon_code.degrees()
        )
    }
}

impl UvoxId {
    pub fn to_vec3(&self) -> [f32; 3] {
        let (x, y, z) = self.to_cartesian();
        [x as f32, y as f32, z as f32]
    }

    pub fn from_vec3(v: [f32; 3]) -> Self {
        let x = v[0] as f64;
        let y = v[1] as f64;
        let z = v[2] as f64;

        let r_m = (x * x + y * y + z * z).sqrt();
        let r_um = (r_m * 1_000_000.0) as i64;

        let lat_rad = (z / r_m).asin();
        let lon_rad = y.atan2(x);

        let lat_code = LatCode((lat_rad.to_degrees() * ANG_SCALE as f64) as i64);
        let lon_code = LonCode((lon_rad.to_degrees() * ANG_SCALE as f64) as i64);

        UvoxId::new(RUm(r_um), lat_code, lon_code)
    }
}

impl UvoxId {
    /// True if this represents the world origin (r = 0)
    pub fn is_origin(&self) -> bool {
        self.r_um == RUm(0)
    }

    /// Unit direction vector from world center
    pub fn unit_vector(&self) -> [f64; 3] {
        let v = self.to_vec3();

        let x = v[0] as f64;
        let y = v[1] as f64;
        let z = v[2] as f64;

        let mag = (x*x + y*y + z*z).sqrt().max(1e-12);

        [x / mag, y / mag, z / mag]
    }



    /// Radial distance from world center (meters)
    pub fn radius_m(&self) -> f64 {
        self.r_um.0 as f64 * 1e-6
    }



}
impl UvoxId {
    /// Approximate local distance in micrometers (valid for small regions)
    pub fn approx_distance_um(&self, other: &UvoxId) -> i64 {
        let dr = (self.r_um.0 - other.r_um.0).abs();

        let dlat = (self.lat_code.0 - other.lat_code.0).abs();
        let dlon = (self.lon_code.0 - other.lon_code.0).abs();

        // Weight angular deltas by radius (very rough but stable)
        let r = self.r_um.0;

        dr + (r * dlat / 1_000_000) + (r * dlon / 1_000_000)
    }
}
