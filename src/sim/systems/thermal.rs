use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::core::TimeDelta,
    matcat::materials::{props_for, default_props},
    geospec::traits::{SurfaceArea, Volume},
    sim::components::thermal::{ThermalData, ThermalExposure}
};
use uuid::Uuid;
use serde_json::json;
use std::f64::consts::PI;
use chrono::{Datelike};

pub struct ThermalSystem;

impl System for ThermalSystem {
    fn name(&self) -> &'static str {
        "ThermalSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };
        let dt_s = clock.step.num_seconds();

        for (id, obj) in &world.objects {
            let uuid = match Uuid::parse_str(id) {
                Ok(u) => u,
                Err(_) => continue,
            };

            let mat = if let Some(mat_id) = &obj.material.matcat_id {
                props_for(mat_id)
            } else {
                default_props()
            };

            let area = obj.shape.surface_area();
            // inside tick()
            let entry = world.thermal_components.entry(uuid).or_insert_with(|| ThermalData {
                temperature_c: 20.0,
                heat_capacity_j_per_kg_k: mat.specific_heat as f64,
                absorptivity: 0.7,
                mass_kg: mat.density as f64 * obj.shape.volume(),
            });

            let exposure = world.thermal_exposure.entry(uuid).or_insert_with(|| ThermalExposure {
                total_energy_j: 0.0,
                average_temperature_c: 20.0,
                cycles: 0,
            });

            // --- realistic per-day heating ---
            let solar_hours = 6.0;
                if let Some(sun) = world.sunlight_components.get(&uuid) {
            let absorbed_energy = sun.irradiance_w_m2 * area * (solar_hours * 3600.0) * entry.absorptivity;

            let delta_t = absorbed_energy / (entry.mass_kg * entry.heat_capacity_j_per_kg_k);
            entry.temperature_c = (entry.temperature_c + delta_t).clamp(-50.0, 100.0);

            exposure.total_energy_j += absorbed_energy;
            exposure.cycles += 1;
            exposure.average_temperature_c +=
                (entry.temperature_c - exposure.average_temperature_c) / exposure.cycles as f64;

            // --- emit both events ---
            events.push(ChronoEvent {
                id: obj.uvoxid.clone(),
                t: TimeDelta::from_ticks(dt_s, "seconds"),
                kind: EventKind::Custom("ThermalUpdate".into()),
                payload: Some(json!({
                    "temperature_c": entry.temperature_c,
                    "mass_kg": entry.mass_kg,
                    "heat_capacity_j_per_kg_k": entry.heat_capacity_j_per_kg_k,
                    "absorptivity": entry.absorptivity
                })),
            });

            events.push(ChronoEvent {
                id: obj.uvoxid.clone(),
                t: TimeDelta::from_ticks(dt_s, "seconds"),
                kind: EventKind::Custom("ThermalExposureUpdate".into()),
                payload: Some(json!({
                    "total_energy_j": exposure.total_energy_j,
                    "average_temperature_c": exposure.average_temperature_c,
                    "cycles": exposure.cycles
                })),
            });
        }

            
        }

        events
    }
}
