use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::core::TimeDelta,
    matcat::materials::props_for,
};
use serde_json::json;
use uuid::Uuid;
use crate::tdt::sim_duration::SimDuration;

/// Tracks UV degradation accumulation per object
#[derive(Debug, Clone)]
pub struct UVDegradationData {
    pub cumulative_uv_j_m2: f64,
    pub severity: f64,
    pub rate_m_per_year: f64,
}

pub struct UVDegradationSystem;

impl UVDegradationSystem {
    /// Convert cumulative UV dose into degradation severity (0–1)
    fn severity_from_dose(dose: f64, resistance: f64) -> f64 {
        // Materials with higher UV resistance tolerate more dose
        let effective_dose = dose / (resistance.clamp(0.01, 1.0) * 1e10);
        effective_dose.min(1.0)
    }
}

impl System for UVDegradationSystem {
    fn name(&self) -> &'static str { "UVDegradationSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let Some(clock) = &world.clock else { return events };

        for (entity_id_str, obj) in &world.objects {
            let uuid = match Uuid::parse_str(entity_id_str) {
                Ok(id) => id,
                Err(_) => continue,
            };

            // Require solar exposure data
            let Some(exposure) = world.solar_exposure_components.get(&uuid) else {
                continue;
            };

            // Material must have a MatCatId
            let props = if let Some(mat_id) = &obj.material.matcat_id {
                props_for(mat_id)
            } else {
                continue;
            };

            let resistance = props.uv_resistance as f64;

            // -----------------------------
            // FIXED: correct field name
            // -----------------------------
            let cumulative_uv = exposure.uv_j_m2;

            let severity = Self::severity_from_dose(cumulative_uv, resistance);

            // Update entry
            let entry = world.uv_degradation_components
                .entry(uuid)
                .or_insert(UVDegradationData {
                    cumulative_uv_j_m2: 0.0,
                    severity: 0.0,
                    rate_m_per_year: 0.0,
                });

            entry.cumulative_uv_j_m2 = cumulative_uv;
            entry.severity = severity;

            // Emit event
            let event_name = if severity >= 1.0 {
                "UVDegradationFailure"
            } else {
                "UVDegradationProgress"
            };

            events.push(ChronoEvent {
                id: obj.uvoxid,
                t: TimeDelta::from_sim_duration(
                    SimDuration::from_ns(clock.step_ns())
                ),
                kind: EventKind::Custom(event_name.into()),
                payload: Some(json!({
                    "date": clock.current_wall_time().to_rfc3339(),

                    "uv_total_j_m2": cumulative_uv,
                    "severity": severity,
                    "resistance": resistance
                })),
            });

        }

        events
    }
}
