use crate::core::uvoxid::UvoxId;
use std::fmt::Debug;

/// Height of the solid surface relative to reference sea level
/// Positive = land
/// Zero = shoreline
/// Negative = ocean basin
pub trait LandHeightField: Send + Sync + Debug {
    fn height_m(&self, uvox: &UvoxId) -> f64;
}
