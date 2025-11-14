use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{
        systems::System,
        world::WorldState,
        components::corrosion::CorrosionData,
    },
    tdt::core::TimeDelta,
    tdt::sim_duration::{SimDuration},
    matcat::materials::props_for,
    objex::systems::mass::derive_mass_from_objex,
};

use uuid::Uuid;
use serde_json::json;

pub struct CorrosionSystem;

impl CorrosionSystem {
    /// Convert meters to millimeters
    fn m_to_mm(m: f64) -> f64 {
        m * 1000.0
    }

    /// Compute corrosion rate (m/year) based on material + environment
    fn rate_m_per_year(resistance: f64, env_factor: f64) -> f64 {
        // 0 resistance → 0.1mm/year
        // 1 resistance → ~0 corrosion
        (1.0 - resistance) * 1e-4 * env_factor
    }
}

impl System for CorrosionSystem {
    fn name(&self) -> &'static str { "CorrosionSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };

        // Convert sim step to fractional years
        let dt_years = clock.step_seconds() / (365.0 * 86400.0);

        for (id_str, obj) in &world.objects {
            let uuid = match Uuid::parse_str(id_str) {
                Ok(id) => id,
                Err(_) => continue,
            };

            // Requires material
            let Some(mat_id) = &obj.material.matcat_id else {
                continue;
            };

            let props = props_for(mat_id);
            let resistance = props.corrosion_resistance as f64;
            let env_factor = 1.0;     // Placeholder for future humidity / salinity / pH modeling

            let rate = Self::rate_m_per_year(resistance, env_factor);

            // Geometry
            let mass = derive_mass_from_objex(obj);
            let area = mass.surface_area_m2;

            // Component entry
            let entry = world.corrosion_components
                .entry(uuid)
                .or_insert(CorrosionData {
                    object_id: uuid,
                    surface_area: area,
                    thickness_loss: 0.0,
                    rate,
                    environment_factor: env_factor as f32,
                    severity: 0.0,
                });

            // Accumulate corrosion
            entry.thickness_loss += rate * dt_years;

            let reference_thickness = 0.01;  // 1 cm
            entry.severity = (entry.thickness_loss / reference_thickness).min(1.0);

            // Human-readable mm
            let loss_mm = Self::m_to_mm(entry.thickness_loss);

            let (event_type, details) = if entry.severity >= 1.0 {
                (
                    "CorrosionFailure",
                    json!({
                        "rate_m_per_year": entry.rate,
                        "severity": entry.severity,
                        "reference_thickness_m": reference_thickness,
                    }),
                )
            } else {
                (
                    "CorrosionProgress",
                    json!({
                        "thickness_loss_m": entry.thickness_loss,
                        "thickness_loss_mm": loss_mm,
                        "severity": entry.severity,
                    }),
                )
            };

            // Emit event using real simtime
            events.push(ChronoEvent {
                id: obj.uvoxid,
                t: TimeDelta::from_sim_duration(SimDuration::from_ns(clock.step_ns())),
                kind: EventKind::Custom(event_type.into()),
                payload: Some(json!({
                    "surface_area_m2": entry.surface_area,
                    "env_factor": entry.environment_factor,
                    "details": details
                })),
            });
        }

        events
    }
}
