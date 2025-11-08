use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::corrosion::CorrosionData},
    tdt::core::TimeDelta,
    matcat::materials::props_for,
    objex::systems::mass::derive_mass,
};
use uuid::Uuid;
use tracing::info;
use serde_json::json;

pub struct CorrosionSystem;

impl System for CorrosionSystem {
    fn name(&self) -> &'static str {
        "CorrosionSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const DT: f64 = 1.0; // seconds per tick (adjust as needed)

        for (entity_id_str, obj) in world.objects.iter() {
            if let Ok(entity_id) = Uuid::parse_str(entity_id_str) {
                if let Some(mat_id) = &obj.material.matcat_id {
                    let props = props_for(mat_id);
                    let resistance = props.corrosion_resistance as f64;

                    // Derive geometry data
                    let geom = derive_mass_from_objex(obj);
                    let surface_area = geom.surface_area_m2;

                    // Initialize or update corrosion state
                    let entry = world.corrosion_components
                        .entry(entity_id)
                        .and_modify(|c| {
                            // Compute new thickness loss
                            c.thickness_loss += c.rate * DT;
                            // Update severity (0.0–1.0)
                            let thickness_ref = 0.01; // 1 cm reference thickness (customize per obj)
                            c.severity = (c.thickness_loss / thickness_ref).min(1.0);
                        })
                        .or_insert_with(|| {
                            // Estimate base corrosion rate (inverse of resistance)
                            let environment_factor = 1.0; // default (no environment modeling yet)
                            let rate = (1.0 - resistance) * 1e-6 * environment_factor; // m/s baseline
                            CorrosionData {
                                object_id: entity_id,
                                surface_area,
                                thickness_loss: 0.0,
                                rate,
                                environment_factor: environment_factor as f32,
                                severity: 0.0,
                            }
                        });

                    // Emit corrosion progression events
                    if entry.severity >= 1.0 {
                        info!("⚠️ {:?} fully corroded!", entity_id);
                        events.push(ChronoEvent {
                            id: obj.uvoxid.clone(),
                            t: TimeDelta::from_ticks(1, "seconds"),
                            kind: EventKind::Custom("CorrosionFailure".into()),
                            payload: Some(json!({
                                "surface_area_m2": entry.surface_area,
                                "rate_m_per_s": entry.rate,
                                "severity": entry.severity,
                            })),
                        });
                    } else {
                        events.push(ChronoEvent {
                            id: obj.uvoxid.clone(),
                            t: TimeDelta::from_ticks(1, "seconds"),
                            kind: EventKind::Custom("CorrosionProgress".into()),
                            payload: Some(json!({
                                "surface_area_m2": entry.surface_area,
                                "thickness_loss_m": entry.thickness_loss,
                                "severity": entry.severity,
                            })),
                        });
                    }
                }
            }
        }

        events
    }
}
