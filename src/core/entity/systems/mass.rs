use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::components::mass::Mass;
use crate::core::physics::units::mass::Kilograms;
use std::f64::consts::PI;

pub fn compute_entity_mass(store: &mut EntityStore) {
    for id in store.actives.keys().copied() {
        let density = match store.densities.get(&id) {
            Some(d) => d.0 .0,
            None => continue,
        };

        // Sphere
        if let Some(radius) = store.radii.get(&id) {
            let r = radius.0 .0;
            let volume = (4.0 / 3.0) * PI * r * r * r;
            let mass = density * volume;

            store.masses.insert(id, Mass(Kilograms(mass)));
            continue;
        }

        // Box / slab
        if let (Some(w), Some(h), Some(t)) = (
            store.widths.get(&id),
            store.heights.get(&id),
            store.thicknesses.get(&id),
        ) {
            let volume = w.0 .0 * h.0 .0 * t.0 .0;
            let mass = density * volume;

            store.masses.insert(id, Mass(Kilograms(mass)));
        }
    }
}
