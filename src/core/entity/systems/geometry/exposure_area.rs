use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::physics::units::area::SquareMeters;
use std::f64::consts::PI;

/// Compute the maximum projected exposure area of an entity.
///
/// This is the area used for radiation coupling before
/// directional projection.
///
/// Returns None if geometry is missing or ambiguous.
pub fn compute_entity_exposure_area(
    entity: EntityId,
    store: &EntityStore,
) -> Option<SquareMeters> {
    // --- Sphere ---
    if let Some(radius) = store.radii.get(&entity) {
        let r = radius.0 .0;
        let area = PI * r * r; // projected disk
        return Some(SquareMeters(area));
    }

    // --- Box / slab ---
    if let (Some(w), Some(h)) = (
        store.widths.get(&entity),
        store.heights.get(&entity),
    ) {
        let area = w.0 .0 * h.0 .0;
        return Some(SquareMeters(area));
    }

    None
}
