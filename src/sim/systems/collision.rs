use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState},
    tdt::{sim_time::SimTime, sim_duration::SimDuration},
    physox::{
        interaction::{restitution, damage}, 
        energy::kinetic_energy
    },
    matcat::materials::{props_for, MatCatId, MatProps},
    uvoxid::units::{um_to_m, um_to_cm, HumanLength},
};
use uuid::Uuid;
use serde_json::json;
use crate::sim::components::fracture::FractureData;

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn name(&self) -> &'static str { "CollisionSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        const EARTH_RADIUS: i64 = 6_371_000_000_000; // µm

        // Pull sim time
        let start = world.sim_time;
        let end = world.sim_time.add(world.sim_delta);

        // === Object–Object collisions ===
        let object_ids: Vec<_> = world.objects.keys().cloned().collect();
        for (i, id_a) in object_ids.iter().enumerate() {
            for id_b in object_ids.iter().skip(i + 1) {
                let obj_a = &world.objects[id_a];
                let obj_b = &world.objects[id_b];
                let dr = (obj_a.uvoxid.r_um - obj_b.uvoxid.r_um).abs();

                let ra = obj_a.shape.approx_radius_um();
                let rb = obj_b.shape.approx_radius_um();

                if dr <= (ra + rb) {
                    // Stop both
                    if let Some(v) = world.velocity_components.get_mut(&Uuid::parse_str(id_a).unwrap()) {
                        v.dr = 0.0; v.dlat = 0.0; v.dlon = 0.0;
                    }
                    if let Some(v) = world.velocity_components.get_mut(&Uuid::parse_str(id_b).unwrap()) {
                        v.dr = 0.0; v.dlat = 0.0; v.dlon = 0.0;
                    }

                    let dr_human = dr.to_human();

                    events.push(ChronoEvent {
                        id: obj_a.uvoxid.clone(),
                        t: end,

                        kind: EventKind::Custom(format!("Collision with {}", id_b)),
                        payload: Some(json!({
                            "object_a": id_a,
                            "object_b": id_b,
                            "r_um": obj_a.uvoxid.r_um,
                            "impact_distance_um": dr,
                            "impact_distance_m": um_to_m(dr),
                            "impact_distance_cm": um_to_cm(dr),
                            "impact_distance_human": dr_human
                        })),
                    });

                    events.push(ChronoEvent {
                        id: obj_b.uvoxid.clone(),
                        t: end,

                        kind: EventKind::Custom(format!("Collision with {}", id_a)),
                        payload: None,
                    });
                }
            }
        }

        // === Object–Ground collisions ===
        for (entity_id, obj) in world.objects.iter_mut() {
            if obj.uvoxid.r_um <= EARTH_RADIUS {
                let entity_uuid = Uuid::parse_str(entity_id).unwrap();

                if let Some(v) = world.velocity_components.get_mut(&entity_uuid) {
                    let pre_impact_speed = v.dr.abs();
                    let mut fractured = false;

                    if let Some(mat_id) = &obj.material.matcat_id {
                        let props = props_for(mat_id);
                        let restitution = crate::matcat::materials::restitution_from_props(&props);
                        let impact_energy = 0.5 * (props.density as f64) * pre_impact_speed.powi(2);

                        if impact_energy > props.fracture_toughness as f64 {
                            fractured = true;
                            let plane = "horizontal".to_string();

                            world.fracture_components.insert(
                                entity_uuid,
                                FractureData {
                                    object_id: entity_uuid,
                                    plane: plane.clone(),
                                    energy: impact_energy,
                                    threshold: props.fracture_toughness,
                                },
                            );

                            events.push(ChronoEvent {
                                id: obj.uvoxid.clone(),
                                t: end,

                                kind: EventKind::Fracture { plane },
                                payload: Some(json!({
                                    "impact_energy": impact_energy,
                                    "threshold": props.fracture_toughness,
                                    "impact_energy_human": format!("{:.2} J", impact_energy),
                                })),
                            });
                        } else {
                            // bounce
                            v.dr = -v.dr * restitution;
                            v.dlat *= restitution * 0.8;
                            v.dlon *= restitution * 0.8;
                        }
                    }

                    if fractured {
                        if let Some(a) = world.acceleration_components.get_mut(&entity_uuid) {
                            a.ar = 0.0;
                            a.alat = 0.0;
                            a.alon = 0.0;
                        }
                    }
                }

                // Snap to ground
                obj.uvoxid.r_um = EARTH_RADIUS;

                events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: end,

                    kind: EventKind::Custom("GroundCollision".into()),
                    payload: Some(json!({
                        "altitude_um": obj.uvoxid.r_um - EARTH_RADIUS,
                        "altitude_m": um_to_m(obj.uvoxid.r_um - EARTH_RADIUS),
                        "altitude_human": (obj.uvoxid.r_um - EARTH_RADIUS).to_human(),
                    })),
                });
            }
        }

        events
    }
}
