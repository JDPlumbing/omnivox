use crate::core::env::fields::{Field, FieldSample};
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::SimDuration;
use crate::core::world::world_env_descriptor::WorldSpace;
use crate::core::env::AtmosphereField;
use crate::core::env::chemistry::OceanChemistry;

// NOTE: This field currently computes atmospheric (gas) pressure only.
// Liquid and solid pressure stacking will be added later.

pub struct PressureField {
    pub space: WorldSpace,
    pub atmosphere: Option<AtmosphereField>,
    pub ocean_chemistry: Option<OceanChemistry>,
    pub step_m: f64,
}

//const WATER_DENSITY_KG_M3: f64 = 1025.0; // seawater

impl Field for PressureField {
    // Primitive phase: pressure contributes nothing
    fn sample(&self, _id: &UvoxId, _time: SimDuration) -> FieldSample {
        FieldSample::default()
    }

    // Derived phase: compute from other fields
   fn derive(
        &self,
        id: &UvoxId,
        _time: SimDuration,
        env: &FieldSample,
    ) -> FieldSample {
        // Vacuum → zero pressure
        if env.medium.is_vacuum() {
            return FieldSample::default();
        }

        let Some(atm) = &self.atmosphere else {
            return FieldSample::default();
        };

        let g = env.gravity_radial.abs();
        let dz = self.step_m;

        let surface_r = self.space.surface_radius_m;
        let current_r = id.r_um.meters();

        // ----------------------------------
        // 1. Atmospheric pressure at surface
        // ----------------------------------
        let max_r = surface_r + atm.max_height_m.unwrap_or(100_000.0);

        let mut surface_pressure = 0.0;
        let mut r = surface_r;

        while r < max_r {
            let rho = atm.density_at_radius(r);
            if rho <= 0.0 {
                break;
            }
            surface_pressure += rho * g * dz;
            r += dz;
        }

        // ----------------------------------
        // 2. Medium-specific pressure
        // ----------------------------------
        let pressure = match env.medium {
            // Gas: pressure above current altitude
            crate::core::env::medium::Medium::Gas => {
                let mut p = 0.0;
                let mut r = current_r.max(surface_r);

                while r < max_r {
                    let rho = atm.density_at_radius(r);
                    if rho <= 0.0 {
                        break;
                    }
                    p += rho * g * dz;
                    r += dz;
                }

                p
            }

            // Liquid: surface pressure + liquid column
            crate::core::env::medium::Medium::Liquid => {
                let rho = match &self.ocean_chemistry {
                    Some(ocean) => ocean.sample(env).density_kg_m3,
                    None => return FieldSample {
                        pressure: surface_pressure,
                        ..Default::default()
                    },
                };

                let depth = (surface_r - current_r).max(0.0);
                surface_pressure + rho * g * depth
            }


            // Solid: for now, surface pressure only
            crate::core::env::medium::Medium::Solid => {
                surface_pressure
            }

            crate::core::env::medium::Medium::Vacuum => 0.0,
        };

        FieldSample {
            pressure,
            ..Default::default()
        }
    }



}
