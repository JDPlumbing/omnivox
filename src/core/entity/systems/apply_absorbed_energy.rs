use crate::shared::entities::EntityStore;
use crate::core::entity::components::internal_energy::InternalEnergy;
use crate::core::entity::components::absorbed_energy::AbsorbedEnergy;
use crate::core::physics::units::energy::Joules;  


pub fn apply_absorbed_energy(
    store: &mut EntityStore,
) {
    let ids: Vec<_> = store.absorbed_energies.keys().cloned().collect();

    for id in ids {
        if !store.is_active(&id) {
            continue;
        }

        let absorbed = store.absorbed_energies[&id];

        let internal = store
            .internal_energies
            .entry(id)
            .or_insert(InternalEnergy { joules: Joules(0.0) });

        internal.joules += absorbed.joules;

        store.absorbed_energies.insert(id, AbsorbedEnergy::default());
    }

}
