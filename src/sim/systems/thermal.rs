use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    objex::matcat::materials::{props_for},
    objex::geospec::traits::{SurfaceArea, Volume},
    uvoxid::units::HumanLength,
};

use crate::sim::{
    systems::System,
    world::WorldState,
    components::thermal::{ThermalData, ThermalExposure},
};

use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ThermalSystem;

impl System for ThermalSystem {
    fn name(&self) -> &'static str {
        "ThermalSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };
        let now = clock.current;

        let dt_s = clock.step_seconds();

        for (entity_id, entity) in world.entities.iter_mut() {

            //---------------------------------------------------------
            // Material props — direct (not Option)
            //---------------------------------------------------------
            let mat_id = &entity.material().matcat_id;
            let mat = props_for(mat_id);

            //---------------------------------------------------------
            // Geometry
            //---------------------------------------------------------
            let shape = entity.shape();
            let area   = shape.surface_area();
            let volume = shape.volume();

            //---------------------------------------------------------
            // Thermal component
            //---------------------------------------------------------
            let thermal = world.components.thermal_components
                .entry(*entity_id)
                .or_insert_with(|| ThermalData {
                    temperature_c: 20.0,
                    heat_capacity_j_per_kg_k: mat.specific_heat as f64,
                    absorptivity: 0.7,
                    mass_kg: (mat.density as f64) * volume,
                });

            //---------------------------------------------------------
            // Exposure tracking
            //---------------------------------------------------------
            let exposure = world.components.thermal_exposure
                .entry(*entity_id)
                .or_insert_with(|| ThermalExposure {
                    total_energy_j: 0.0,
                    average_temperature_c: 20.0,
                    cycles: 0,
                });

            //---------------------------------------------------------
            // Environmental conditions
            //---------------------------------------------------------
            let sunlight = world.components.sunlight_components.get(entity_id);
            let ambient_c: f64 = 15.0;

            let mut net_energy: f64 = 0.0;

            //---------------------------------------------------------
            // Radiative heating
            //---------------------------------------------------------
            if let Some(sun) = sunlight {
                let absorbed = sun.irradiance_w_m2 * area * thermal.absorptivity * dt_s;
                net_energy += absorbed;
                exposure.total_energy_j += absorbed;
            }

            //---------------------------------------------------------
            // Cooling (radiative + convective)
            //---------------------------------------------------------
            let emissivity = 0.9;
            let sigma = 5.67e-8;

            // Explicit f64 types
            let t_k: f64   = thermal.temperature_c + 273.15;
            let amb_k: f64 = ambient_c + 273.15;

            let q_rad  = emissivity * sigma * (t_k.powi(4) - amb_k.powi(4));
            let h_conv = 10.0;
            let q_conv = h_conv * (thermal.temperature_c - ambient_c).max(0.0);

            let cooling = (q_rad + q_conv) * area * dt_s;
            net_energy -= cooling;

            //---------------------------------------------------------
            // Update temperature
            //---------------------------------------------------------
            let delta_t =
                net_energy / (thermal.mass_kg * thermal.heat_capacity_j_per_kg_k);

            thermal.temperature_c =
                (thermal.temperature_c + delta_t).clamp(-80.0, 130.0);

            //---------------------------------------------------------
            // Exposure averaging
            //---------------------------------------------------------
            exposure.cycles += 1;
            exposure.average_temperature_c +=
                (thermal.temperature_c - exposure.average_temperature_c)
                / exposure.cycles as f64;

            //---------------------------------------------------------
            // Events
            //---------------------------------------------------------
            events.push(
                ChronoEvent::new(
                    entity.entity_id,
                    entity.world_id,
                    now,
                    EventKind::Custom("ThermalUpdate".into())
                )
                .with_payload(json!({
                    "temperature_c": thermal.temperature_c,
                    "net_energy_j": net_energy,
                    "mass_kg": thermal.mass_kg,
                    "heat_capacity_j_per_kg_k": thermal.heat_capacity_j_per_kg_k,
                    "absorptivity": thermal.absorptivity,
                }))
            );

            events.push(
                ChronoEvent::new(
                    entity.entity_id,
                    entity.world_id,
                    now,
                    EventKind::Custom("ThermalExposureUpdate".into())
                )
                .with_payload(json!({
                    "total_energy_j": exposure.total_energy_j,
                    "average_temperature_c": exposure.average_temperature_c,
                    "cycles": exposure.cycles,
                }))
            );
        }

        events
    }
}
