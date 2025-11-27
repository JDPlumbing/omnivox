use crate::sim::systems::System;
use crate::core::chronovox::{ChronoEvent, EventKind};
use crate::sim::world::WorldState;

use crate::sim::components::{
    solar_exposure::SolarExposure,
    sun_damage::SunDamage
};

#[derive(Default, Debug)]
pub struct SunDamageSystem;


impl System for SunDamageSystem {
    fn name(&self) -> &'static str {
        "SunDamageSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let dt_s = world.sim_delta.as_secs_f64();

        for (id, exposure) in world.components.solar_exposure.iter() {

            let dmg = world.components.sun_damage
                .entry(*id)
                .or_insert_with(SunDamage::new);

            dmg.total_irradiance_j_m2 += exposure.irradiance_w_m2 * dt_s;
            dmg.total_uv_j_m2 += exposure.uv_intensity * dt_s;

            dmg.total_temp_delta += exposure.temp_delta_c * dt_s;

            let sign = if exposure.temp_delta_c >= 0.0 { 1 } else { -1 };
            if sign != dmg.last_temp_sign {
                dmg.thermal_cycles += 1.0;
                dmg.last_temp_sign = sign;
            }

            events.push(ChronoEvent::new(
                *id,
                world.entities[id].world_id,
                world.sim_time,
                EventKind::Custom("SunDamageIntegration".into())
            ));
        }

        events
    }
}
