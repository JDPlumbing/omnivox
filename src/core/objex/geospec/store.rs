use std::collections::HashMap;
use uuid::Uuid;
use crate::core::objex::geospec::GeoSpec;


pub struct GeoSpecStore {
    inner: HashMap<Uuid, GeoSpec>,
}

impl GeoSpecStore {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn insert(&mut self, id: Uuid, spec: GeoSpec) {
        self.inner.insert(id, spec);
    }

    pub fn get(&self, id: &Uuid) -> Option<&GeoSpec> {
        self.inner.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Uuid, &GeoSpec)> {
        self.inner.iter()
    }
}
