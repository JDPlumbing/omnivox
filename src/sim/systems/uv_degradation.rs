use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    objex::matcat::materials::props_for,
};

use crate::sim::{
    systems::System,
    world::WorldState,
    components::uv_degradation::UVDegradationData,
};

use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UVDegradationSystem;

impl UVDegradationSystem {
    /// Convert cumulative UV dose into degradation severity (0–1)
    fn severity_from_dose(dose: f64, resistance: f64) -> f64 {
        // Avoid divide-by-zero and nonlinear scaling
        let effective = dose / (resistance.clamp(0.01, 1.0) * 1e10);
        effective.min(1.0)
    }
}

impl System for UVDegradationSystem {
    fn name(&self) -> &'static str { "UVDegradationSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = vec![];

        let Some(clock) = &world.clock else { return events; };
        let now = clock.current;

        for (entity_id, entity) in world.entities.iter() {

            // ---- solar exposure required ----
            let Some(exposure) = world.components.solar_exposure_components.get(entity_id)
            else {
                continue;
            };

            // ---- material: ALWAYS present ----
            let mat_id = &entity.material().matcat_id;
            let mat_props = props_for(mat_id);

            let resistance = mat_props.uv_resistance as f64;
            let cumulative_uv = exposure.uv_j_m2;

            // Compute new severity
            let severity = Self::severity_from_dose(cumulative_uv, resistance);

            // ---- Update component ----
            let entry = world.components
                .uv_degradation_components
                .entry(*entity_id)
                .or_insert(UVDegradationData {
                    cumulative_uv_j_m2: 0.0,
                    severity: 0.0,
                    rate_m_per_year: 0.0,
                });

            entry.cumulative_uv_j_m2 = cumulative_uv;
            entry.severity = severity;

            // ---- Event type ----
            let event_name = if severity >= 1.0 {
                "UVDegradationFailure"
            } else {
                "UVDegradationProgress"
            };

            // ---- Emit event ----
            events.push(
                ChronoEvent::new(
                    entity.entity_id,
                    entity.world_id,
                    now,
                    EventKind::Custom(event_name.into())
                )
                .with_payload(json!({
                    "uv_total_j_m2": cumulative_uv,
                    "severity": severity,
                    "resistance": resistance,
                    "timestamp": clock.current_wall_time().to_rfc3339(),
                }))
            );
        }

        events
    }
}
