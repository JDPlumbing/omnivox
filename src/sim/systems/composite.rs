use crate::{
    objex::core::CompositeObject,
    sim::{systems::System, world::WorldState},
    chronovox::ChronoEvent,
    objex::systems::mechanical::MechanicalProps,
};

pub struct CompositeSystem;

impl System for CompositeSystem {
    fn name(&self) -> &'static str {
        "composite"
    }

    fn tick(&mut self, world: &mut WorldState) -> Vec<ChronoEvent> {
        let mut events = Vec::new();

        for (id, comp) in &world.composite_components {
            // Placeholder — using density as a rough proxy for "mass"
            let mech = comp.total_mass() as f32;

            // ✅ Insert a valid MechanicalProps struct, not just a f32
            world.mechanical_components.insert(
                *id,
                MechanicalProps {
                    youngs_modulus: mech,      // junk placeholder until real data
                    hardness: 0.0,
                    fracture_toughness: 0.0,
                    inertia: mech as f64,      // just to fill required field
                },
            );
        }

        events
    }
}
