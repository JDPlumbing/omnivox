use crate::core::simulation::sim_engine::SimulationEngine;
use crate::core::tdt::SimTime;
use crate::shared::EntityStore;

use crate::core::entity::id::EntityId;
use crate::core::entity::components::{
    geometry_parts::radius::Radius,
    materials::{
        density::Density,
        thermal::specific_heat::SpecificHeat,
        emissivity::Emissivity,
    },
    internal_energy::InternalEnergy,
    temperature::Temperature,
    active::Active,
};

use crate::core::physics::units::{
    length::Meters,
    density::KilogramsPerCubicMeter,
    energy::Joules,
    temperature::Kelvin,
    irradiance::WattsPerSquareMeter,
    specific_heat::JoulesPerKilogramKelvin,
};
use crate::core::physics::units::acceleration::MetersPerSecondSquared;
use crate::core::entity::components::entity_environment_sample::EntityEnvironmentSample;
use crate::core::environment::conditions::EnvironmentConditions;
use crate::core::worlds::systems::gravity::LocalENU;
use crate::core::physics::constants::universal::STEFAN_BOLTZMANN;
use std::f64::consts::PI;

#[test]
fn spherical_entity_temperature_converges_to_radiative_equilibrium() {
    // --- Simulation ---
    let mut engine = SimulationEngine::new(
        SimTime::from_seconds_f64(0.0),
        60_000_000_000, // 1 minute per tick
        EntityStore::default(),
    );

    let entity = EntityId::new();
    let radius = 1.0; // m
    let density = 1000.0; // kg/m³
    let specific_heat = 1000.0; // J/(kg·K)
    let emissivity = 0.9; // dimensionless
    let insolation = 1361.0; // W/m² (solar constant)
    
        // 🔓 mutable borrow STARTS here
   {
        let store = &mut engine.state.entities;

        store.add_radius(entity, Radius(Meters(radius)));
        store.add_density(entity, Density(KilogramsPerCubicMeter(density)));
        store.add_specific_heat(entity, SpecificHeat(JoulesPerKilogramKelvin(specific_heat)));
        store.add_emissivity(entity, Emissivity(emissivity));
        store.add_internal_energy(entity, InternalEnergy { joules: Joules(0.0) });
        store.add_temperature(entity, Temperature(Kelvin(1.0)));
        store.actives.insert(entity, Active);

        store.entity_environment_samples.insert(
            entity,
            EntityEnvironmentSample {
                env: EnvironmentConditions {
                    insolation: WattsPerSquareMeter(insolation),
                    ..Default::default()
                },
                gravity_enu: LocalENU {
                    east: MetersPerSecondSquared(0.0),
                    north: MetersPerSecondSquared(0.0),
                    up: MetersPerSecondSquared(0.0),
                },
            },
        );
    } // 🔓 mutable borrow ENDS here

    use crate::core::entity::systems::exposure::accumulate_exposure;
    use crate::core::physics::units::time::Seconds;
    use crate::core::entity::systems::accumulate_absorbed_energy::accumulate_absorbed_energy;
    use crate::core::entity::systems::apply_absorbed_energy::apply_absorbed_energy;
    use crate::core::entity::systems::radiative_cooling::apply_radiative_cooling;
    use crate::core::entity::systems::temperature::update_temperature_from_internal_energy;
    use crate::core::entity::systems::mass::compute_entity_mass;

    

    let store = &mut engine.state.entities;
    compute_entity_mass(store);
    for step in 0..20_000 {
        accumulate_exposure(store, Seconds(60.0));
        accumulate_absorbed_energy(store);
        apply_absorbed_energy(store);
        apply_radiative_cooling(store, Seconds(60.0));
        update_temperature_from_internal_energy(store);

        if step % 1000 == 0 {
            let e = store.internal_energies[&entity].joules.0;
            let t = store.temperatures[&entity].0 .0;
            println!("step {}: E = {} J, T = {} K", step, e, t);
        }
    }



    let temp_sim = store.temperatures[&entity].0 .0;


    // --- Analytic equilibrium ---
    let a_proj = PI * radius * radius;
    let a_surface = 4.0 * PI * radius * radius;

    let t_eq = (insolation * a_proj / (emissivity * STEFAN_BOLTZMANN * a_surface))
        .powf(0.25);

    // --- Assert convergence ---
    let error = (temp_sim - t_eq).abs();

    assert!(
        error < 1.0,
        "Temperature did not converge: sim={}K expected={}K error={}",
        temp_sim,
        t_eq,
        error
    );
}
