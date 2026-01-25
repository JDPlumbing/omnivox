use crate::core::uvoxid::UvoxId;
use crate::core::env::land::height_field::LandHeightField;
use std::fmt::Debug;

#[derive(Debug)]
pub struct NoiseLand {
    /// Amplitude in meters (e.g. 4000 = ±4 km)
    pub amplitude_m: f64,

    /// Frequency (lower = bigger continents)
    pub frequency: f64,

    /// Bias (negative = more ocean)
    pub bias: f64,
}

impl NoiseLand {
    pub fn earth_like() -> Self {
        Self {
            amplitude_m: 4000.0,
            frequency: 0.5,
            bias: -0.3,
        }
    }
}

/// Very simple smooth noise from a 3D vector
fn smooth_noise(x: f64, y: f64, z: f64) -> f64 {
    // Deterministic hash-like noise
    let n = (x * 12.9898 + y * 78.233 + z * 37.719).sin() * 43758.5453;
    n.fract() * 2.0 - 1.0
}

impl LandHeightField for NoiseLand {
    fn height_m(&self, uvox: &UvoxId) -> f64 {
        let lat = uvox.lat_code.radians();
        let lon = uvox.lon_code.radians();

        // Unit sphere coordinates
        let x = lat.cos() * lon.cos();
        let y = lat.cos() * lon.sin();
        let z = lat.sin();

        let n = smooth_noise(
            x * self.frequency,
            y * self.frequency,
            z * self.frequency,
        );

        let shaped = (n + self.bias).clamp(-1.0, 1.0);

        shaped * self.amplitude_m
    }
}
