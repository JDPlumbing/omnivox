use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UvoxId {
    pub r_um: i64,
    pub lat_code: i64,
    pub lon_code: i64,
}

impl UvoxId {
    pub fn new(r_um: i64, lat_code: i64, lon_code: i64) -> Self {
        Self { r_um, lat_code, lon_code }
    }
}
