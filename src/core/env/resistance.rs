use crate::core::tdt::SimDuration;
use crate::core::uvoxid::UvoxId;
use crate::core::env::fields::{Field, FieldSample};

pub struct ResistanceField {
    pub resistance: f64, // N·s/m or arbitrary units
}

impl Field for ResistanceField {
    fn sample(&self, _id: &UvoxId, _time: SimDuration) -> FieldSample {
        FieldSample {
            resistance: self.resistance,
            ..Default::default()
        }
    }
}
