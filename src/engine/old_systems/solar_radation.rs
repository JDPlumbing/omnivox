use crate::engine::systems::System;
use crate::engine::world::WorldState;
use crate::core::chronovox::ChronoEvent;

use crate::core::physox::astronomy::topocentric::sun_topocentric;
use crate::engine::components::{SolarRadiation, SolarExposure};

use std::f64::consts::PI;

#[derive(Default, Debug)]
pub struct SolarRadiationSystem;

impl System for SolarRadiationSystem {
    fn name(&self) -> &'static str { "SolarRadiationSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        let Some(clock) = &world.clock else { return events; };
        let now = clock.current;

        // ---------------------------------------------------------
        // Locate Sun (first entity with SolarRadiation)
        // ---------------------------------------------------------
        let Some((&sun_id, sun_rad)) =
            world.components.solar_radiation.iter().next()
        else {
            return events;
        };

        let Some(sun_entity) = world.entities.get(&sun_id) else {
            return events;
        };

        // Typed radius → meters
        let r_m = sun_entity.position.r_um.meters();

        // Avoid division by zero
        if r_m <= 0.0 {
            return events;
        }

        // 1/r² irradiance falloff
        let irradiance_at_earth =
            sun_rad.luminosity_w / (4.0 * PI * r_m * r_m);

        // ---------------------------------------------------------
        // Compute exposure on each entity
        // ---------------------------------------------------------
        for (id, entity) in world.entities.iter() {
            if *id == sun_id { continue; }

            let observer = entity.position;

            // Compute solar topocentric geometry
            let topo = sun_topocentric(observer, now);

            let irr_factor = topo.irradiance_factor.max(0.0);
            if irr_factor <= 0.0 {
                world.components.solar_exposure.remove(id);
                continue;
            }

            // Local irradiance
            let irr_local = irradiance_at_earth * irr_factor;

            world.components.solar_exposure.insert(
                *id,
                SolarExposure {
                    irradiance_w_m2: irr_local,
                    uv_intensity: irr_local * sun_rad.uv_fraction,
                    temp_delta_c: irr_local * 0.0002, // simple heat absorption
                }
            );
        }

        events
    }
}
