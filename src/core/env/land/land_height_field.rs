use crate::core::env::fields::{Field, FieldSample};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::SimDuration;
use std::sync::Arc;
use super::height_field::LandHeightField;

pub struct LandHeightEnvField {
    pub land: Arc<dyn LandHeightField>,
}

impl Field for LandHeightEnvField {
    fn sample(&self, uvox: &UvoxId, _time: SimDuration) -> FieldSample {
        FieldSample {
            land_height_m: self.land.height_m(uvox),
            ..Default::default()
        }
    }
}
