use crate::core::{
    chronovox::{ChronoEvent, EventKind},
    tdt::time_delta::TimeDelta,
    objex::geospec::Shape,
};

use crate::sim::{
    systems::System,
    world::WorldState,
    components::velocity::Velocity,
};

use uuid::Uuid;
use rand::Rng;
use serde_json::{json, Value};
use serde::{Serialize, Deserialize};
use crate::core::id::entity_id::EntityId;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FractureSystem;

impl System for FractureSystem {
    fn name(&self) -> &'static str { "FractureSystem" }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();
        let mut spawned_fragments = Vec::new();
        let mut parents_to_remove = Vec::new();

        let Some(clock) = &world.clock else { return events };
        let _dt = TimeDelta::from_sim_duration(clock.step);

        let mut rng = rand::rng();

        //
        // Scan for fractured parents
        //
        for (parent_id, parent) in world.entities.iter() {
            // Skip fragments (they already contain "_frag" in metadata)
            let name_opt = parent
                .metadata
                .get("name")
                .and_then(|v| v.as_str());

            if let Some(name) = name_opt {
                if name.contains("_frag") {
                    continue;
                }
            }

            // Check fracture component
            let Some(fract) = world.components.fracture_components.get(&parent_id) else {
                continue;
            };

            let total_energy = fract.energy;
            let child_count = rng.random_range(2..=4);
            let base_scatter = (total_energy / child_count as f64)
                .sqrt()
                .clamp(0.1, 500.0);

            parents_to_remove.push(*parent_id);

            //
            // Spawn fragments
            //
            for i in 0..child_count {
                let mut frag = parent.clone();

                //
                // Give fragment a name
                //
                let base_name = name_opt.unwrap_or("object");
                let frag_name = format!("{base_name}_frag{}", i + 1);

                // Ensure metadata is an object before inserting
                if !frag.metadata.is_object() {
                    frag.metadata = json!({});
                }
                if let Some(obj) = frag.metadata.as_object_mut() {
                    obj.insert("name".to_string(), Value::String(frag_name.clone()));
                }

                //
                // Assign a new entity ID
                //
                let new_id = EntityId::new(0,0);
                frag.id = new_id;

                //
                // Resize spherical fragments (if sphere)
                //
                if let Shape::Sphere(s) = frag.blueprint.shape.clone() {
                    frag.blueprint.shape = Shape::Sphere(
                        crate::core::objex::geospec::shapes::Sphere {
                            radius: s.radius / (child_count as f64).sqrt(),
                        }
                    );
                }

                //
                // Random displacement (µm)
                //
                frag.position.r_um += rng.random_range(-2_000_000..2_000_000);

                //
                // Velocity scatter
                //
                let scatter = base_scatter * rng.random_range(0.5..1.5);
                let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

                world.components.velocity_components.insert(
                    new_id,
                    Velocity {
                        dr: sign * scatter * 1e-6,
                        dlat: rng.random_range(-0.1..0.1) * scatter * 1e-7,
                        dlon: rng.random_range(-0.1..0.1) * scatter * 1e-7,
                    },
                );

                //
                // Stage the fragment for insertion after the loop
                //
                spawned_fragments.push((new_id, frag.clone()));

                //
                // Emit spawn event
                //
                events.push(
                    ChronoEvent::new(
                        frag.id,
                        frag.world_id,
                        clock.current,
                        EventKind::Spawn
                    )
                    .with_payload(Value::String(format!(
                        "Fragment created (energy {:.2})",
                        total_energy
                    )))
                );
            }

            //
            // Emit despawn event for the fractured parent
            //
            events.push(
                ChronoEvent::new(
                    parent.id,
                    parent.world_id,
                    clock.current,
                    EventKind::Despawn
                )
                .with_payload(Value::String(format!(
                    "Fractured into {} fragments (energy {:.2})",
                    child_count, total_energy
                )))
            );
        }

        //
        // Remove all fractured parents
        //
        for parent_id in parents_to_remove {
            world.entities.remove(&parent_id);
            world.components.velocity_components.remove(&parent_id);
            world.components.acceleration_components.remove(&parent_id);
            world.components.fracture_components.remove(&parent_id);
        }

        //
        // Insert new fragments
        //
        for (id, frag) in spawned_fragments {
            world.entities.insert(id, frag);
        }

        events
    }
}
