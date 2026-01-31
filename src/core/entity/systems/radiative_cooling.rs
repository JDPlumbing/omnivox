use crate::shared::entities::entity_store::EntityStore;

use crate::core::entity::components::internal_energy::InternalEnergy;
use crate::core::entity::components::temperature::Temperature;
use crate::core::entity::components::material::emissivity::Emissivity;

use crate::core::physics::constants::universal::STEFAN_BOLTZMANN as SIGMA;
use crate::core::physics::units::energy::Joules;
use crate::core::physics::units::time::Seconds;

use crate::core::entity::systems::geometry::surface_area::compute_entity_surface_area;

/// Apply radiative cooling using the Stefan–Boltzmann law.
///
/// Physics:
///     P = ε σ A T⁴
///     ΔE = P · Δt
///
/// This subtracts energy from InternalEnergy.
/// Temperature is NOT updated here — it is derived later.
pub fn apply_radiative_cooling(
    store: &mut EntityStore,
    dt: Seconds,
) {
    for (id, temperature) in store.temperatures.iter() {
        if !store.is_active(id) {
            continue;
        }

        let InternalEnergy { joules } = match store.internal_energies.get(id) {
            Some(e) => *e,
            None => continue,
        };

        let emissivity = store
            .emissivities
            .get(id)
            .map(|e| e.0)
            .unwrap_or(1.0);

        let area = match compute_entity_surface_area(*id, store) {
            Some(a) => a.0,
            None => continue,
        };

        let t = temperature.0 .0;

        // Stefan–Boltzmann law
        let power_watts = emissivity * SIGMA * area * t.powi(4);

        // Energy lost this tick
        let delta_e = power_watts * dt.0;

        // Clamp so we never go negative
        let new_energy = (joules.0 - delta_e).max(0.0);

        store.internal_energies.insert(
            *id,
            InternalEnergy {
                joules: Joules(new_energy),
            },
        );
    }
}
