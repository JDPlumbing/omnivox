use crate::sim::systems::System;
use crate::sim::world::WorldState;
use crate::core::chronovox::ChronoEvent;

use crate::core::env::solar::solar_flux;
use crate::core::physox::astronomy::topocentric::sun_topocentric;
use crate::sim::components::{SolarExposure, SolarRadiation};

#[derive(Default, Debug)]
pub struct SolarExposureSystem;

impl System for SolarExposureSystem {
    fn name(&self) -> &'static str { "SolarExposureSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = vec![];
        let Some(clock) = &world.clock else { return events; };
        let now = clock.current;

        // Find Sun
        let Some((&sun_id, _sun_rad)) =
            world.components.solar_radiation.iter().next()
        else { return events; };

        let Some(sun_entity) = world.entities.get(&sun_id) else { return events; };
        let sun_pos = sun_entity.position;

        // Compute solar flux at this radial distance (W/m²)
        let base_flux = solar_flux(&sun_pos);

        for (id, entity) in world.entities.iter() {
            if *id == sun_id {
                continue;
            }

            let topo = sun_topocentric(entity.position, now);

            let cos_factor = topo.irradiance_factor.max(0.0);

            if cos_factor <= 0.0 {
                world.components.solar_exposure.remove(id);
                continue;
            }

            let local_flux = base_flux * cos_factor;

            world.components.solar_exposure.insert(
                *id,
                SolarExposure {
                    irradiance_w_m2: local_flux,
                    uv_intensity: local_flux * world.components.solar_radiation[&sun_id].uv_fraction,
                    temp_delta_c: local_flux * 0.0002, // simple absorption/heat model
                }
            );
        }

        events
    }
}
