use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::systems::geometry::geometry_exposure_area::compute_geometry_exposure_area;
use crate::core::physics::units::energy::Joules;

pub fn accumulate_absorbed_energy(
    store: &mut EntityStore,
) {
    let entities: Vec<_> = store.exposures.keys().cloned().collect();

    for entity in entities {
        if !store.is_active(&entity) {
            continue;
        }

        let exposure = match store.exposures.get(&entity) {
            Some(e) => e.radiant.0, // J / m²
            None => continue,
        };

        // ✅ FETCH GEOMETRY
        let geometry = match store.geometries.get(&entity) {
            Some(g) => g,
            None => continue,
        };

        // ✅ PURE GEOMETRY HELPER
        let area = match compute_geometry_exposure_area(geometry) {
            Some(a) => a.0, // m²
            None => continue,
        };

        let delta_energy = exposure * area; // J

        let entry = store
            .absorbed_energies
            .entry(entity)
            .or_default();

        entry.joules += Joules(delta_energy);
    }
}
