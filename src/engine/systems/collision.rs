use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    physox::{
        interaction::{restitution as compute_restitution, damage},
        energy::kinetic_energy,
    },
    objex::matcat::materials::{props_for},
    objex::matcat::properties::restitution_from_props,
    uvoxid::units::{um_to_m, um_to_cm, HumanLength},
};

use crate::engine::{
    components::fracture::FractureData,
    systems::System,
    world::WorldState,
};

use serde::{Serialize, Deserialize};
use serde_json::json;
use crate::core::id::entity_id::EntityId;
use crate::core::uvoxid::RUm;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CollisionSystem;

impl System for CollisionSystem {
    fn name(&self) -> &'static str { "CollisionSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_RADIUS: i64 = 6_371_000_000_000; // µm

        // Time window for this tick
        let end = world.sim_time.add(world.sim_delta);

        //
        // === ENTITY–ENTITY COLLISIONS ===
        //
        let entity_ids: Vec<EntityId> = world.entities.keys().cloned().collect();

        for (i, id_a) in entity_ids.iter().enumerate() {
            for id_b in entity_ids.iter().skip(i + 1) {
                let a = &world.entities[&id_a];
                let b = &world.entities[&id_b];

                let ra = a.shape().radius_um();
                let rb = b.shape().radius_um();

                // unwrap RUm into µm
                let dr = (a.position.r_um.0 - b.position.r_um.0).abs();

                if dr <= (ra + rb) {
                    // Stop velocities
                    if let Some(v) = world.components.velocity_components.get_mut(&id_a) {
                        v.dr = 0.0; v.dlat = 0.0; v.dlon = 0.0;
                    }
                    if let Some(v) = world.components.velocity_components.get_mut(&id_b) {
                        v.dr = 0.0; v.dlat = 0.0; v.dlon = 0.0;
                    }

                    let dr_human = dr.to_human();

                    // Event: A hits B
                    events.push(
                        ChronoEvent::new(
                            a.id, a.world_id, end,
                            EventKind::Custom(format!("Collision with {}", id_b)),
                        )
                        .with_payload(json!({
                            "entity_a": id_a,
                            "entity_b": id_b,
                            "impact_distance_um": dr,
                            "impact_distance_m": um_to_m(dr),
                            "impact_distance_cm": um_to_cm(dr),
                            "impact_distance_human": dr_human,
                        }))
                    );

                    // Event: B hits A
                    events.push(
                        ChronoEvent::new(
                            b.id, b.world_id, end,
                            EventKind::Custom(format!("Collision with {}", id_a)),
                        )
                    );
                }
            }
        }

        //
        // === ENTITY–GROUND COLLISIONS ===
        //
        for (id, entity) in world.entities.iter_mut() {
            if entity.position.r_um.0 <= EARTH_RADIUS {
                // Process velocity → bounce or fracture
                if let Some(v) = world.components.velocity_components.get_mut(&id) {
                    let pre_speed = v.dr.abs();
                    let mut fractured = false;

                    let mat_id = entity.material().matcat_id;
                    let props = props_for(&mat_id);
                    let restitution = restitution_from_props(&props);

                    let impact_energy =
                        0.5 * (props.density as f64) * pre_speed.powi(2);

                    if impact_energy > props.fracture_toughness as f64 {
                        fractured = true;

                        world.components.fracture_components.insert(
                            *id,
                            FractureData {
                                entity_id: *id,
                                plane: "horizontal".into(),
                                energy: impact_energy,
                                threshold: props.fracture_toughness,
                            },
                        );

                        events.push(
                            ChronoEvent::new(
                                entity.id, entity.world_id, end,
                                EventKind::Fracture { plane: "horizontal".into() },
                            )
                            .with_payload(json!({
                                "impact_energy": impact_energy,
                                "threshold": props.fracture_toughness,
                                "impact_energy_human": format!("{:.2} J", impact_energy),
                            }))
                        );

                    } else {
                        // Simple bounce
                        v.dr   = -v.dr * restitution;
                        v.dlat =  v.dlat * restitution * 0.8;
                        v.dlon =  v.dlon * restitution * 0.8;
                    }

                    if fractured {
                        if let Some(a) =
                            world.components.acceleration_components.get_mut(&id)
                        {
                            a.ar = 0.0;
                            a.alat = 0.0;
                            a.alon = 0.0;
                        }
                    }
                }

                //
                // Snap to ground
                //
                entity.position.r_um = RUm(EARTH_RADIUS);

                let altitude_um = entity.position.r_um.0 - EARTH_RADIUS;

                events.push(
                    ChronoEvent::new(
                        entity.id, entity.world_id, end,
                        EventKind::Custom("GroundCollision".into()),
                    )
                    .with_payload(json!({
                        "altitude_um": altitude_um,
                        "altitude_m": um_to_m(altitude_um),
                        "altitude_human": altitude_um.to_human(),
                    }))
                );
            }
        }

        events
    }
}
