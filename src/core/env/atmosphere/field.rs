use crate::core::env::fields::{Field, FieldSample};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::SimDuration;
//use crate::core::medium::Medium;
use crate::core::world::world_env_descriptor::{WorldSpace, AtmosphereModel};

#[derive(Clone)]
pub struct AtmosphereField {
    pub planet_radius_m: f64,
    pub sea_level_density: f64,
    pub scale_height_m: f64,
    pub max_height_m: Option<f64>,
}

impl AtmosphereField {
    pub fn from_model(space: &WorldSpace, model: &AtmosphereModel) -> Self {
        Self {
            planet_radius_m: space.surface_radius_m,
            sea_level_density: model.sea_level_density,
            scale_height_m: model.scale_height_m,
            max_height_m: model.max_height_m,
        }
    }
}
impl Field for AtmosphereField {
    fn sample(&self, id: &UvoxId, _time: SimDuration) -> FieldSample {
        let r = id.r_um.meters();
        let height_m = r - self.planet_radius_m;

        let density = if height_m < 0.0 {
            // Below surface
            0.0
        } else if let Some(max_h) = self.max_height_m {
            if height_m > max_h {
                0.0
            } else {
                self.sea_level_density * (-height_m / self.scale_height_m).exp()
            }
        } else {
            self.sea_level_density * (-height_m / self.scale_height_m).exp()
        };

        FieldSample {
            density: density,
            ..Default::default()
        }
    }
}

impl AtmosphereField {
    /// Density at absolute radius from planet center (meters)
    pub fn density_at_radius(&self, r: f64) -> f64 {
        let height_m = r - self.planet_radius_m;
        
        if height_m < 0.0 {
            // Below surface
            0.0
        } else if let Some(max_h) = self.max_height_m {
            if height_m > max_h {
                0.0
            } else {
                self.sea_level_density * (-height_m / self.scale_height_m).exp()
            }
        } else {
            self.sea_level_density * (-height_m / self.scale_height_m).exp()
        }
    }
}
