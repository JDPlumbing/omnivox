use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::Velocity},
    tdt::core::TimeDelta,
    uvoxid::units::{um_to_m, HumanLength},
};
use serde_json::json;

pub struct MovementSystem;

impl System for MovementSystem {
    fn name(&self) -> &'static str {
        "MovementSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut triggered_events = Vec::new();

        for (entity_id, velocity) in world.velocity_components.iter() {
            if let Some(obj) = world.objects.get_mut(&entity_id.to_string()) {
                // Compute displacement in µm (still your internal integer grid)
                let dr_total = velocity.dr as i64;

                obj.uvoxid.r_um += dr_total;
                obj.uvoxid.lat_code += velocity.dlat as i64;
                obj.uvoxid.lon_code += velocity.dlon as i64;

                // Convert to meters for reporting
                let displacement_m = um_to_m(dr_total);

                // Add a human-readable field
                let human_disp = dr_total.to_human();

                // Emit movement event
                triggered_events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: world.clock.as_ref().unwrap().current, 

                    kind: EventKind::Move {
                        dr: velocity.dr as i64,
                        dlat: velocity.dlat as i64,
                        dlon: velocity.dlon as i64,
                    },
                    payload: Some(json!({
                        "displacement_um": dr_total,
                        "displacement_m": displacement_m,
                        "displacement_human": human_disp,
                    })),
                });

            }
        }

        triggered_events
    }
}
