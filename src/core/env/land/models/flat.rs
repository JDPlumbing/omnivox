use crate::core::uvoxid::UvoxId;
use crate::core::env::land::height_field::LandHeightField;

/// Flat reference surface (no land, no basins)
#[derive(Debug)]
pub struct FlatLand;

impl LandHeightField for FlatLand {
    fn height_m(&self, _uvox: &UvoxId) -> f64 {
        -1000.0 //NOTE: THIS IS A TEST VALUE
    }
}
