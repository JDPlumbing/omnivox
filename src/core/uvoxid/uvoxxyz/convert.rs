use crate::uvoxid::UvoxId;
use crate::uvoxxyz::types::{Cartesian, CoordSystem};
use std::f64::consts::PI;

const DEG_SCALE: f64 = 180e6; // 180 * 10^6 microdegrees

/// Extension trait for converting between UvoxId and Cartesian coordinates.
pub trait UvoxIdExt {
    fn to_cartesian(&self, system: CoordSystem) -> Cartesian;
    fn from_cartesian(cart: Cartesian, system: CoordSystem, frame_id: i64) -> UvoxId;
}

impl UvoxIdExt for UvoxId {
    /// Convert UvoxId → Cartesian (Math or Graphics).
    fn to_cartesian(&self, system: CoordSystem) -> Cartesian {
        let r = self.r_um as f64 * 1e-6; // µm → meters
        let lat_rad = (self.lat_code as f64) * PI / DEG_SCALE;
        let lon_rad = (self.lon_code as f64) * PI / DEG_SCALE;

        match system {
            CoordSystem::Math => {
                let x = r * lon_rad.cos() * lat_rad.cos();
                let y = r * lon_rad.sin() * lat_rad.cos();
                let z = r * lat_rad.sin();
                Cartesian { x, y, z }
            }
            CoordSystem::Graphics => {
                let x = r * lon_rad.cos() * lat_rad.cos();
                let z = r * lon_rad.sin() * lat_rad.cos();
                let y = r * lat_rad.sin();
                Cartesian { x, y, z }
            }
        }
    }

    /// Convert Cartesian → UvoxId (Math or Graphics).
    fn from_cartesian(cart: Cartesian, system: CoordSystem, frame_id: i64) -> UvoxId {
        let r = (cart.x.powi(2) + cart.y.powi(2) + cart.z.powi(2)).sqrt();

        let (lat_rad, lon_rad) = match system {
            CoordSystem::Math => {
                let lat = (cart.z / r).asin();
                let lon = cart.y.atan2(cart.x);
                (lat, lon)
            }
            CoordSystem::Graphics => {
                let lat = (cart.y / r).asin();
                let lon = cart.z.atan2(cart.x);
                (lat, lon)
            }
        };

        let r_um = (r * 1e6).round() as i64;
        let lat_code = (lat_rad * DEG_SCALE / PI).round() as i64;
        let lon_code = (lon_rad * DEG_SCALE / PI).round() as i64;

        UvoxId {
            frame_id,
            r_um,
            lat_code,
            lon_code,
        }
    }
}
