// core/entity/systems/surface_area.rs
use crate::shared::entities::entity_store::EntityStore;
use crate::core::entity::id::EntityId;
use crate::core::entity::components::geometry::surface_area::SurfaceArea;
use crate::core::entity::systems::geometry::compute_entity_surface_area;

pub fn update_entity_surface_area(
    store: &mut EntityStore,
) {
    let entities: Vec<EntityId> = store.actives.keys().cloned().collect();

    for entity in entities {
        if let Some(area) = compute_entity_surface_area(entity, store) {
            store.surface_areas.insert(entity, SurfaceArea(area));
        }
    }
}
