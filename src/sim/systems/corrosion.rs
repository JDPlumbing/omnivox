use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::corrosion::CorrosionData, clock::SimClock},
    tdt::core::TimeDelta,
    matcat::materials::props_for,
    objex::systems::mass::derive_mass_from_objex,
};
use uuid::Uuid;
use tracing::info;
use serde_json::json;
use chrono::Duration;

pub struct CorrosionSystem;

impl CorrosionSystem {
    /// Convert meters to millimeters
    fn m_to_mm(m: f64) -> f64 {
        m * 1000.0
    }

    /// Compute corrosion rate (m/year) based on material + environment.
    fn rate_m_per_year(resistance: f64, env_factor: f64) -> f64 {
        (1.0 - resistance) * 1e-4 * env_factor // baseline: 0.1 mm per year for fully exposed metal
    }

    /// Compute time to failure (years) given reference thickness and corrosion rate
    fn time_to_failure_years(reference_thickness_m: f64, rate_m_per_year: f64) -> f64 {
        if rate_m_per_year <= 0.0 {
            f64::INFINITY
        } else {
            reference_thickness_m / rate_m_per_year
        }
    }
}

impl System for CorrosionSystem {
    fn name(&self) -> &'static str {
        "CorrosionSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        // ✅ Pull simulated time step from the world’s clock if available
        let dt_days = world.clock.as_ref().map(|c| c.step.num_days()).unwrap_or(30);
        let dt_years = dt_days as f64 / 365.0;

        for (entity_id_str, obj) in &world.objects {
            let entity_id = match Uuid::parse_str(entity_id_str) {
                Ok(id) => id,
                Err(_) => continue,
            };

            if let Some(mat_id) = &obj.material.matcat_id {
                let props = props_for(mat_id);
                let resistance = props.corrosion_resistance as f64;
                let env_factor = 1.0; // placeholder until we model humidity, pH, etc.
                let rate = Self::rate_m_per_year(resistance, env_factor); // m/year

                // get geometry info
                let geom = derive_mass_from_objex(obj);
                let surface_area = geom.surface_area_m2;

                // grab or insert corrosion record
                let entry = world.corrosion_components.entry(entity_id).or_insert_with(|| CorrosionData {
                    object_id: entity_id,
                    surface_area,
                    thickness_loss: 0.0,
                    rate,
                    environment_factor: env_factor as f32,
                    severity: 0.0,
                });

                // increment corrosion thickness over this simulated step
                entry.thickness_loss += rate * dt_years;
                let reference_thickness = 0.01; // 1 cm baseline
                entry.severity = (entry.thickness_loss / reference_thickness).min(1.0);

                // Derived values
                let loss_mm = Self::m_to_mm(entry.thickness_loss);
                let loss_str = format!("{:.4} mm", loss_mm);

                // emit event when we cross thresholds
                let (event_type, extra) = if entry.severity >= 1.0 {
                    let time_to_fail_years = Self::time_to_failure_years(reference_thickness, rate);
                    let time_to_fail_str = if time_to_fail_years.is_finite() {
                        format!("{:.2} years", time_to_fail_years)
                    } else {
                        "∞ (no corrosion)".to_string()
                    };

                    info!("⚠️ {:?} fully corroded after ~{}", entity_id, time_to_fail_str);
                    (
                        "CorrosionFailure",
                        json!({
                            "rate_m_per_year": entry.rate,
                            "severity": entry.severity,
                            "time_to_failure_years": time_to_fail_years,
                            "time_to_failure_human": time_to_fail_str
                        }),
                    )
                } else {
                    (
                        "CorrosionProgress",
                        json!({
                            "thickness_loss_m": entry.thickness_loss,
                            "thickness_loss_mm": loss_mm,
                            "thickness_loss_human": loss_str,
                            "severity": entry.severity
                        }),
                    )
                };

                events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: TimeDelta::from_ticks(dt_days * 86400, "seconds"),
                    kind: EventKind::Custom(event_type.into()),
                    payload: Some(json!({
                        "surface_area_m2": entry.surface_area,
                        "date": world.clock.as_ref().map(|c| c.current.to_string()),
                        "details": extra
                    })),
                });
            }
        }

        events
    }
}
