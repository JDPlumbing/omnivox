use crate::{
    chronovox::{ChronoEvent, EventKind},
    objex::core::Objex,
    sim::{systems::System, world::WorldState, components::Velocity},
    tdt::core::TimeDelta,
    sim::components::fracture::FractureData,
};
use uuid::Uuid;
use serde_json::Value;
use rand::Rng;

pub struct FractureSystem;

impl System for FractureSystem {
    fn name(&self) -> &'static str {
        "FractureSystem"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let mut to_spawn = Vec::new();
        let mut to_remove = Vec::new();
        let mut rng = rand::rng();

        for (entity_id, obj) in world.objects.iter() {
            let entity_uuid = Uuid::parse_str(entity_id).unwrap();

            if obj.name.contains("_fracture") {
                continue;
            }

            // check fracture components to see if this object fractured
            if let Some(fracture_data) = world.fracture_components.get(&entity_uuid) {
                to_remove.push(entity_id.clone());

                let child_count = rng.random_range(2..=4); // slightly variable number of pieces
                let total_energy = fracture_data.energy;
                let base_scatter = (total_energy / child_count as f64).sqrt().clamp(0.1, 500.0);

                for i in 0..child_count {
                    let mut frag = obj.clone();
                    frag.name = format!("{}_frag{}", obj.name, i + 1);

                    // shrink fragments proportionally to number
                    frag.shape = match &obj.shape {
                        crate::objex::core::Shape::Sphere(s) => {
                            crate::objex::core::Shape::Sphere(
                                crate::geospec::shapes::Sphere { radius: s.radius / (child_count as f64).sqrt() }
                            )
                        }
                        _ => obj.shape.clone(),
                    };

                    // add a random offset up to ±2 mm (2e6 µm)
                    frag.uvoxid.r_um += rng.random_range(-2_000_000..2_000_000);

                    let new_id = Uuid::new_v4().to_string();

                    // scatter velocity proportional to fracture energy
                    let scatter_factor = base_scatter * rng.random_range(0.5..1.5);
                    let dir_sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };
                    let dr = dir_sign * scatter_factor * 1e-6; // µm/ns scale
                    let dlat = rng.random_range(-0.1..0.1) * scatter_factor * 1e-7;
                    let dlon = rng.random_range(-0.1..0.1) * scatter_factor * 1e-7;

                    world.velocity_components.insert(
                        Uuid::parse_str(&new_id).unwrap(),
                        Velocity { dr, dlat, dlon },
                    );

                    to_spawn.push((new_id.clone(), frag.clone()));

                    events.push(ChronoEvent {
                        id: frag.uvoxid.clone(),
                        t: TimeDelta::from_ticks(1, "nanoseconds"),
                        kind: EventKind::Spawn,
                        payload: Some(Value::String(format!(
                            "Fragment of {} (scattered with {:.2} energy)",
                            entity_id, fracture_data.energy
                        ))),
                    });
                }

                // emit despawn for parent
                events.push(ChronoEvent {
                    id: obj.uvoxid.clone(),
                    t: TimeDelta::from_ticks(1, "nanoseconds"),
                    kind: EventKind::Despawn,
                    payload: Some(Value::String(format!(
                        "Fractured into {} scattered fragments (energy {:.2})",
                        child_count, fracture_data.energy
                    ))),
                });
            }
        }

        // remove fractured parents
        for id in to_remove {
            world.objects.remove(&id);
            world.fracture_components.remove(&Uuid::parse_str(&id).unwrap());
        }

        // insert new fragments
        for (id, frag) in to_spawn {
            world.objects.insert(id, frag);
        }

        events
    }
}
