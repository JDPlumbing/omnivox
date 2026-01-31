use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::physics::units::area::SquareMeters;
use std::f64::consts::PI;

/// Compute total surface area of an entity.
///
/// Used for radiative emission (cooling).
pub fn compute_entity_surface_area(
    entity: EntityId,
    store: &EntityStore,
) -> Option<SquareMeters> {
    // --- Sphere ---
    if let Some(radius) = store.radii.get(&entity) {
        let r = radius.0 .0;
        let area = 4.0 * PI * r * r;
        return Some(SquareMeters(area));
    }

    // --- Box ---
    if let (Some(w), Some(h), Some(t)) = (
        store.widths.get(&entity),
        store.heights.get(&entity),
        store.thicknesses.get(&entity),
    ) {
        let area =
            2.0 * (
                w.0 .0 * h.0 .0 +
                w.0 .0 * t.0 .0 +
                h.0 .0 * t.0 .0
            );
        return Some(SquareMeters(area));
    }

    None
}
