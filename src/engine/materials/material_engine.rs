pub struct MaterialEngine<'a> {
    store: &'a mut EntityStore,
}

impl<'a> MaterialEngine<'a> {
    pub fn new(store: &'a mut EntityStore) -> Self {
        Self { store }
    }
    pub fn add_hardness(
        &mut self,
        entity: EntityId,
        hardness: Hardness,
    ) -> Result<(), MaterialError> {
        if self.store.viscosities.contains_key(&entity) {
            return Err(MaterialError::IncompatibleProperties);
        }

        self.store.add_hardness(entity, hardness);
        Ok(())
    }

    pub fn add_viscosity(
        &mut self,
        entity: EntityId,
        viscosity: Viscosity,
    ) -> Result<(), MaterialError> {
        if self.store.hardnesses.contains_key(&entity) {
            return Err(MaterialError::IncompatibleProperties);
        }

        self.store.add_viscosity(entity, viscosity);
        Ok(())
    }


}
