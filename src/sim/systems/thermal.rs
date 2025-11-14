use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::core::TimeDelta,
    tdt::sim_time::SimDuration,
    matcat::materials::{props_for, default_props},
    geospec::traits::{SurfaceArea, Volume},
    sim::components::thermal::{ThermalData, ThermalExposure},
};
use uuid::Uuid;
    use serde_json::json;

pub struct ThermalSystem;

impl System for ThermalSystem {
    fn name(&self) -> &'static str {
        "ThermalSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };

        // REAL sim time delta in seconds
        let dt_s = clock.step_seconds();

        for (id_str, obj) in &world.objects {
            let uuid = match Uuid::parse_str(id_str) {
                Ok(u) => u,
                Err(_) => continue,
            };

            // Get material props
            let mat = if let Some(mat_id) = &obj.material.matcat_id {
                props_for(mat_id)
            } else {
                default_props()
            };

            // Geometry
            let area = obj.shape.surface_area();
            let volume = obj.shape.volume();

            // Init components if missing
            let entry = world.thermal_components.entry(uuid).or_insert_with(|| ThermalData {
                temperature_c: 20.0,
                heat_capacity_j_per_kg_k: mat.specific_heat as f64,
                absorptivity: 0.7,
                mass_kg: (mat.density as f64) * volume,
            });

            let exposure = world.thermal_exposure.entry(uuid).or_insert_with(|| ThermalExposure {
                total_energy_j: 0.0,
                average_temperature_c: 20.0,
                cycles: 0,
            });

            // --- Real sunlight input ---
            let sunlight = world.sunlight_components.get(&uuid);
            let ambient_c = 15.0;

            let mut net_energy: f64 = 0.0;

            if let Some(sun) = sunlight {
                // Radiative heating from sunlight
                let absorbed = sun.irradiance_w_m2 * area * entry.absorptivity * dt_s;
                net_energy += absorbed;
                exposure.total_energy_j += absorbed;
            }

            // --- Cooling (always on, but stronger at night) ---
            let emissivity = 0.9;
            let sigma = 5.67e-8; // Stefan–Boltzmann constant

            let t_k = entry.temperature_c + 273.15_f64;
            let amb_k = ambient_c + 273.15_f64;


            // Radiative cooling
            let q_rad = emissivity * sigma * (t_k.powi(4) - amb_k.powi(4));

            // Convective cooling (rough model)
            let h_conv = 10.0;
            let q_conv = h_conv * (entry.temperature_c - ambient_c).max(0.0);

            let cooling = (q_rad + q_conv) * area * dt_s;
            net_energy -= cooling;

            // Temperature update
            let delta_t = net_energy / (entry.mass_kg * entry.heat_capacity_j_per_kg_k);
            entry.temperature_c = (entry.temperature_c + delta_t).clamp(-80.0, 130.0);

            // Update averages
            exposure.cycles += 1;
            exposure.average_temperature_c +=
                (entry.temperature_c - exposure.average_temperature_c) / exposure.cycles as f64;


            // Emit thermal update event
            events.push(ChronoEvent {
                id: obj.uvoxid,
                t: TimeDelta::from_sim_duration(SimDuration::from_ns(clock.step_ns)),
                kind: EventKind::Custom("ThermalUpdate".into()),
                payload: Some(json!({
                    "temperature_c": entry.temperature_c,
                    "net_energy_j": net_energy,
                    "mass_kg": entry.mass_kg,
                    "heat_capacity_j_per_kg_k": entry.heat_capacity_j_per_kg_k,
                    "absorptivity": entry.absorptivity,
                })),
            });

            // Emit exposure event
            events.push(ChronoEvent {
                id: obj.uvoxid,
                t: TimeDelta::from_sim_duration(SimDuration::from_ns(clock.step_ns)),
                kind: EventKind::Custom("ThermalExposureUpdate".into()),
                payload: Some(json!({
                    "total_energy_j": exposure.total_energy_j,
                    "average_temperature_c": exposure.average_temperature_c,
                    "cycles": exposure.cycles,
                })),
            });
        }

        events
    }
}
