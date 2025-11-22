use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    uvoxid::units::{um_to_m, HumanLength},
};

use crate::sim::{
    systems::System,
    world::WorldState,
};

use serde_json::json;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MovementSystem;

impl System for MovementSystem {
    fn name(&self) -> &'static str {
        "MovementSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut triggered_events = Vec::new();

        let Some(clock) = &world.clock else {
            return triggered_events;
        };

        for (entity_id, velocity) in world.components.velocity_components.iter() {
            if let Some(entity) = world.entities.get_mut(entity_id) {

                //
                // Convert f64 → i64 movement units
                //
                let dr_i64   = velocity.dr   as i64;
                let dlat_i64 = velocity.dlat as i64;
                let dlon_i64 = velocity.dlon as i64;

                //
                // Apply movement on the µm-grid position
                //
                entity.uvoxid.r_um     += dr_i64;
                entity.uvoxid.lat_code += dlat_i64;
                entity.uvoxid.lon_code += dlon_i64;

                //
                // Movement metrics (nice to have)
                //
                let displacement_m  = um_to_m(dr_i64);
                let human_disp      = dr_i64.to_human();

                //
                // Emit correct ChronoEvent using new API
                //
                triggered_events.push(
                    ChronoEvent::new(
                        entity.entity_id,
                        entity.world_id,
                        clock.current,
                        EventKind::Move {
                            dr: dr_i64,
                            dlat: dlat_i64,
                            dlon: dlon_i64,
                        }
                    )
                    .with_payload(json!({
                        "displacement_um": dr_i64,
                        "displacement_m": displacement_m,
                        "displacement_human": human_disp,
                    }))
                );
            }
        }

        triggered_events
    }
}
