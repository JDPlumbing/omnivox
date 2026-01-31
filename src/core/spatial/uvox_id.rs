use serde::{Serialize, Deserialize};

/// Absolute radial distance from world center, in micrometers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RUm(pub i64);

/// Latitude code (angular, scaled integer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LatCode(pub i64);

/// Longitude code (angular, scaled integer)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LonCode(pub i64);


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UvoxId {
    pub r_um: RUm,
    pub lat: LatCode,
    pub lon: LonCode,
}
impl UvoxId {
    pub fn new(r_um: RUm, lat: LatCode, lon: LonCode) -> Self {
        Self { r_um, lat, lon }
    }
}
