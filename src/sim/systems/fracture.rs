use crate::{
    chronovox::{ChronoEvent, EventKind},
    sim::{systems::System, world::WorldState, components::Velocity},
    sim::components::fracture::FractureData,
    tdt::core::TimeDelta,
};
use uuid::Uuid;
use rand::Rng;
use serde_json::Value;

pub struct FractureSystem;

impl System for FractureSystem {
    fn name(&self) -> &'static str { "FractureSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let mut spawned = Vec::new();
        let mut removed = Vec::new();
        let mut rng = rand::rng();

        let Some(clock) = &world.clock else { return events };
        let dt = TimeDelta::from_sim_duration(clock.step);

        for (_id_str, obj) in world.objects.iter() {
            let parent_id = obj.entity_id;

            if obj.name.contains("_fracture") {
                continue;
            }

            if let Some(fract) = world.fracture_components.get(&parent_id) {
                removed.push(parent_id);

                let child_count = rng.random_range(2..=4);
                let total_energy = fract.energy;
                let base_scatter = (total_energy / child_count as f64)
                    .sqrt()
                    .clamp(0.1, 500.0);

                for i in 0..child_count {
                    let mut frag = obj.clone();
                    frag.name = format!("{}_frag{}", obj.name, i + 1);

                    // geometry scaling
                    if let crate::objex::core::Shape::Sphere(s) = frag.shape {
                        frag.shape = crate::objex::core::Shape::Sphere(
                            crate::geospec::shapes::Sphere {
                                radius: s.radius / (child_count as f64).sqrt(),
                            }
                        );
                    }

                    // position scatter
                    frag.uvoxid.r_um += rng.random_range(-2_000_000..2_000_000);

                    let new_id = Uuid::new_v4();
                    frag.entity_id = new_id;

                    // velocity scatter
                    let scatter = base_scatter * rng.random_range(0.5..1.5);
                    let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

                    world.velocity_components.insert(
                        new_id,
                        Velocity {
                            dr: sign * scatter * 1e-6,
                            dlat: rng.random_range(-0.1..0.1) * scatter * 1e-7,
                            dlon: rng.random_range(-0.1..0.1) * scatter * 1e-7,
                        },
                    );

                    spawned.push((new_id, frag.clone()));

                    events.push(ChronoEvent {
                        id: frag.uvoxid,
                        t: world.clock.as_ref().unwrap().current,

                        kind: EventKind::Spawn,
                        payload: Some(Value::String(format!(
                            "Fragment of {} (scattered with {:.2} energy)",
                            parent_id, total_energy
                        ))),
                    });
                }

                events.push(ChronoEvent {
                    id: obj.uvoxid,
                    t: world.clock.as_ref().unwrap().current,

                    kind: EventKind::Despawn,
                    payload: Some(Value::String(format!(
                        "Fractured into {} fragments (energy {:.2})",
                        child_count, total_energy
                    ))),
                });
            }
        }

        // apply changes
        for id in removed {
            world.objects.remove(&id.to_string());
            world.fracture_components.remove(&id);
        }

        for (id, frag) in spawned {
            world.objects.insert(id.to_string(), frag);
        }

        events
    }
}
