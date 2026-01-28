// infra/world_sources/state/entity_store_snapshot.rs
#[derive(Serialize, Deserialize)]
pub struct EntityStoreSnapshot {
    pub lengths: HashMap<String, Length>,
    pub radii: HashMap<String, Radius>,
    pub thicknesses: HashMap<String, Thickness>,
    pub widths: HashMap<String, Width>,
    pub heights: HashMap<String, Height>,
    pub densities: HashMap<String, Density>,
    pub hardnesses: HashMap<String, Hardness>,
    pub viscosities: HashMap<String, Viscosity>,
    pub conductivities: HashMap<String, Conductivity>,
    pub times: HashMap<String, Time>,
    pub notes: HashMap<String, Note>,
    pub world_memberships: HashMap<String, WorldMembership>,
    pub positions: HashMap<String, Position>,
    pub spawned_ats: HashMap<String, SpawnedAt>,
    pub despawned_ats: HashMap<String, DespawnedAt>,
    pub actives: Vec<String>,
}
impl From<&EntityStore> for EntityStoreSnapshot {
    fn from(store: &EntityStore) -> Self {
        Self {
            lengths: store.lengths.iter()
                .map(|(id, v)| (id.to_string(), v.clone()))
                .collect(),
            // repeat
            actives: store.actives.keys()
                .map(|id| id.to_string())
                .collect(),
        }
    }
}

impl From<EntityStoreSnapshot> for EntityStore {
    fn from(snapshot: EntityStoreSnapshot) -> Self {
        let mut store = EntityStore::default();

        store.lengths = snapshot.lengths.into_iter()
            .map(|(id, v)| (id.parse().unwrap(), v))
            .collect();
        // repeat

        for id in snapshot.actives {
            store.actives.insert(id.parse().unwrap(), Active);
        }

        store
    }
}
