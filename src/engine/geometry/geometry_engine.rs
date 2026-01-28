use crate::shared::entities::EntityStore;
use crate::core::EntityId;
use crate::core::components::geometry::{Length, 
                                            Radius,     
                                            Thickness, 
                                            Width, 
                                            Height
                                        };


pub struct GeometryEngine<'a> {
    store: &'a mut EntityStore,
}

impl<'a> GeometryEngine<'a> {
    pub fn new(store: &'a mut EntityStore) -> Self {
        Self { store }
    }
    pub fn set_thickness(
        &mut self,
        entity: EntityId,
        thickness: Thickness,
    ) -> Result<(), GeometryError> {
        if let Some(radius) = self.store.radii.get(&entity) {
            if thickness.0 >= radius.0 {
                return Err(GeometryError::InvalidDimensions);
            }
        }

        self.store.thicknesses.insert(entity, thickness);
        Ok(())
    }
    pub fn set_radius(
        &mut self,
        entity: EntityId,
        radius: Radius,
    ) -> Result<(), GeometryError> {
        if radius.0 <= 0.0 {
            return Err(GeometryError::InvalidDimensions);
        }

        self.store.radii.insert(entity, radius);
        Ok(())
    }


}
