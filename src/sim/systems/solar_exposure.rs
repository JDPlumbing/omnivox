use crate::sim::systems::System;
use crate::sim::world::WorldState;
use crate::core::chronovox::{ChronoEvent, EventKind};

use crate::sim::components::{SolarExposure, SunDamage};

/// Integrates instantaneous solar exposure into long-term damage.
/// This runs AFTER SolarRadiationSystem.
#[derive(Default, Debug)]
pub struct SolarExposureSystem;

impl System for SolarExposureSystem {
    fn name(&self) -> &'static str { "SolarExposureSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // Time step in seconds
        let dt_s = world.sim_delta.as_secs_f64();

        for (id, exposure) in world.components.solar_exposure.iter() {
            // Create or fetch long-term accumulator
            let dmg = world.components.sun_damage
                .entry(*id)
                .or_insert_with(|| crate::sim::components::SunDamage::new());

            // Integrate irradiance (W/m² → J/m²)
            dmg.total_irradiance_j_m2 += exposure.irradiance_w_m2 * dt_s;

            // Integrate UV
            dmg.total_uv_j_m2 += exposure.uv_intensity * dt_s;

            // Integrate thermal burden
            dmg.total_temp_delta += exposure.temp_delta_c;

            // detect thermal cycles (sign-change tracking)
            let sign = exposure.temp_delta_c.signum() as i8;
            if dmg.last_temp_sign != 0 && sign != dmg.last_temp_sign {
                dmg.thermal_cycles += 1.0;
            }
            dmg.last_temp_sign = sign;

            // Add event
            events.push(
                ChronoEvent::new(
                    *id,
                    world.meta.id,
                    world.sim_time,
                    EventKind::Custom("SolarExposureIntegrated".into()),
                )
            );
        }

        events
    }
}
