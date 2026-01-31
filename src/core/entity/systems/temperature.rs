use crate::shared::entities::entity_store::EntityStore;

use crate::core::entity::components::internal_energy::InternalEnergy;
use crate::core::entity::components::temperature::Temperature;
use crate::core::entity::components::mass::Mass;
use crate::core::entity::components::material::thermal::specific_heat::SpecificHeat;

use crate::core::physics::units::temperature::Kelvin;

/// Derive entity temperature from internal energy.
///
/// Physics:
///     T = E / (m * c)
///
/// where:
///     E = internal energy (J)
///     m = mass (kg)
///     c = specific heat capacity (J / kg·K)
///
/// This is a derived-state system:
/// - no time integration
/// - no accumulation
/// - always recomputed from current energy
pub fn update_temperature_from_internal_energy(
    store: &mut EntityStore,
) {
    for (id, energy) in store.internal_energies.iter() {
        if !store.is_active(id) {
            continue;
        }

        let mass = match store.masses.get(id) {
            Some(m) => m.0 .0,
            None => continue,
        };

        let specific_heat = match store.specific_heats.get(id) {
            Some(c) => c.0 .0,
            None => continue,
        };

        let temperature = energy.joules.0 / (mass * specific_heat);

        store.temperatures.insert(
            *id,
            Temperature(Kelvin(temperature.max(0.0))),
        );
    }
}
