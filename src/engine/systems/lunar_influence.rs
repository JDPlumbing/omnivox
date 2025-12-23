use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::core::TimeDelta,
    geospec::bodies::moon::Moon,
};
use serde_json::json;
use chrono::Datelike;

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LunarInfluence {
    pub total_illumination_j_m2: f64,
    pub total_tidal_energy_j: f64,
    pub cycles: u64,
}

pub struct LunarInfluenceSystem {
    pub moon: Moon,
}

impl LunarInfluenceSystem {
    pub fn new() -> Self {
        Self { moon: Moon::new() }
    }
}

impl System for LunarInfluenceSystem {
    fn name(&self) -> &'static str {
        "LunarInfluenceSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };
        let dt_s = clock.step.num_seconds();

        // --- update moon position ---
        let day_of_year = clock.now.ordinal() as f64;
        self.moon.update(day_of_year);

        // --- per-object coupling ---
        for (id, _obj) in &world.objects {
            let exposure = world.lunar_exposure.entry(id.clone()).or_default();

            let illumination = self.moon.irradiance_w_m2 * (dt_s as f64);
            let tidal_energy = self.moon.tidal_force_n_per_kg * (dt_s as f64);

            exposure.total_illumination_j_m2 += illumination;
            exposure.total_tidal_energy_j += tidal_energy;
            exposure.cycles += 1;

            events.push(ChronoEvent {
                id: id.clone(),
                t: TimeDelta::from_ticks(dt_s, "seconds"),
                kind: EventKind::Custom("LunarUpdate".into()),
                payload: Some(json!({
                    "phase": self.moon.phase,
                    "irradiance_w_m2": self.moon.irradiance_w_m2,
                    "tidal_force_n_per_kg": self.moon.tidal_force_n_per_kg,
                })),
            });

            events.push(ChronoEvent {
                id: id.clone(),
                t: TimeDelta::from_ticks(dt_s, "seconds"),
                kind: EventKind::Custom("LunarExposureUpdate".into()),
                payload: Some(json!({
                    "total_illumination_j_m2": exposure.total_illumination_j_m2,
                    "total_tidal_energy_j": exposure.total_tidal_energy_j,
                    "cycles": exposure.cycles,
                })),
            });
        }

        events
    }
}
