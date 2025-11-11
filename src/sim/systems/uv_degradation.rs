use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::core::TimeDelta,
    matcat::materials::props_for,
};
use serde_json::json;
use uuid::Uuid;

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
        // materials with higher resistance need more UV to degrade
        let effective_dose = dose / (resistance.clamp(0.01, 1.0) * 1e10);
        effective_dose.min(1.0)
    }
}

impl System for UVDegradationSystem {
    fn name(&self) -> &'static str {
        "UVDegradationSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        let clock = match &world.clock {
            Some(c) => c,
            None => return events,
        };

        for (entity_id_str, obj) in &world.objects {
            let uuid = match Uuid::parse_str(entity_id_str) {
                Ok(id) => id,
                Err(_) => continue,
            };

            // requires solar exposure data
            let Some(exposure) = world.solar_exposure_components.get(&uuid) else {
                continue;
            };

            let props = if let Some(mat_id) = &obj.material.matcat_id {
                props_for(mat_id)
            } else {
                continue;
            };

            let resistance = props.uv_resistance as f64;
            let cumulative_uv = exposure.cumulative_uv_j_m2;

            let severity = Self::severity_from_dose(cumulative_uv, resistance);

            let entry = world.uv_degradation_components.entry(uuid).or_insert_with(|| UVDegradationData {
                cumulative_uv_j_m2: 0.0,
                severity: 0.0,
                rate_m_per_year: 0.0,
            });

            entry.cumulative_uv_j_m2 = cumulative_uv;
            entry.severity = severity;

            // emit events
            if severity >= 1.0 {
                events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: TimeDelta::from_ticks(clock.step.num_seconds(), "seconds"),
                    kind: EventKind::Custom("UVDegradationFailure".into()),
                    payload: Some(json!({
                        "date": clock.current.to_rfc3339(),
                        "uv_total_j_m2": cumulative_uv,
                        "severity": severity,
                        "resistance": resistance
                    })),
                });
            } else {
                events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: TimeDelta::from_ticks(clock.step.num_seconds(), "seconds"),
                    kind: EventKind::Custom("UVDegradationProgress".into()),
                    payload: Some(json!({
                        "date": clock.current.to_rfc3339(),
                        "uv_total_j_m2": cumulative_uv,
                        "severity": severity,
                        "resistance": resistance
                    })),
                });
            }
        }

        events
    }
}
