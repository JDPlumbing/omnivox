use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    objex::matcat::materials::props_for,
    objex::systems::mass::derive_mass,
    objex::core::Objex,
};

use crate::engine::{
    systems::System,
    world::WorldState,
    components::corrosion::CorrosionData,
};

use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CorrosionSystem;

impl CorrosionSystem {
    /// Convert meters to millimeters
    fn m_to_mm(m: f64) -> f64 {
        m * 1000.0
    }

    /// Compute corrosion rate (m/year)
    fn rate_m_per_year(resistance: f64, env_factor: f64) -> f64 {
        (1.0 - resistance) * 1e-4 * env_factor
    }
}

impl System for CorrosionSystem {
    fn name(&self) -> &'static str { "CorrosionSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = vec![];

        let Some(clock) = &world.clock else {
            return events;
        };

        let now = clock.current;
        let dt_years = clock.step_seconds() / (365.0 * 86400.0);

        for (id, entity) in world.entities.iter() {

            //---------------------------------------------------------
            // Material properties — NO Option!
            //---------------------------------------------------------
            let mat_id = &entity.material();
            let mat_props = props_for(mat_id);

            //---------------------------------------------------------
            // Build Objex blueprint
            //---------------------------------------------------------
            let objex = Objex {
                shape: entity.shape().clone(),
                material: entity.material().clone(),
            };

            //---------------------------------------------------------
            // Mass model → we only need surface area
            //---------------------------------------------------------
            let mass_info = derive_mass(&objex);
            let area = mass_info.surface_area_m2;

            //---------------------------------------------------------
            // Compute corrosion rate
            //---------------------------------------------------------
            let env_factor = 1.0;
            let rate = Self::rate_m_per_year(
                mat_props.corrosion_resistance as f64,
                env_factor,
            );

            //---------------------------------------------------------
            // Initialize/update component
            //---------------------------------------------------------
            let entry = world.components.corrosion_components
                .entry(*id)
                .or_insert(CorrosionData {
                    entity_id: *id,
                    surface_area: area,
                    thickness_loss: 0.0,
                    rate,
                    environment_factor: env_factor as f32,
                    severity: 0.0,
                });

            //---------------------------------------------------------
            // Update corrosion state
            //---------------------------------------------------------
            entry.thickness_loss += rate * dt_years;

            let reference_thickness = 0.01; // 1 cm steel
            entry.severity = (entry.thickness_loss / reference_thickness).min(1.0);

            let loss_mm = Self::m_to_mm(entry.thickness_loss);

            //---------------------------------------------------------
            // Choose event type
            //---------------------------------------------------------
            let (event_name, details) = if entry.severity >= 1.0 {
                (
                    "CorrosionFailure",
                    json!({
                        "rate_m_per_year": entry.rate,
                        "severity": entry.severity,
                        "reference_thickness_m": reference_thickness
                    })
                )
            } else {
                (
                    "CorrosionProgress",
                    json!({
                        "thickness_loss_m": entry.thickness_loss,
                        "thickness_loss_mm": loss_mm,
                        "severity": entry.severity
                    })
                )
            };

            //---------------------------------------------------------
            // Emit event
            //---------------------------------------------------------
            events.push(
                ChronoEvent::new(
                    entity.id,
                    entity.world_id,
                    now,
                    EventKind::Custom(event_name.into())
                )
                .with_payload(json!({
                    "surface_area_m2": entry.surface_area,
                    "env_factor": entry.environment_factor,
                    "details": details
                }))
            );
        }

        events
    }
}
