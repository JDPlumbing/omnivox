use crate::shared::entities::entity_store::EntityStore;
use crate::core::physics::units::{time::Seconds,
                                        };
use crate::core::render::primitives::RenderPrimitive;   
use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

use crate::core::simulation::state::SimulationState;
use crate::core::cosmic::state::CosmicState;
use crate::core::worlds::state::WorldState;
use crate::core::environment::state::EnvironmentState;

use crate::core::entity::systems::env::sample_env_for_active_entities::sample_environment_for_active_entities;
use crate::core::entity::systems::update_entity_volume::update_entity_volume;
use crate::core::entity::systems::update_entity_surface_area::update_entity_surface_area;

use crate::core::entity::systems::compute_entity_mass::compute_entity_mass;
use crate::core::entity::systems::compute_entity_weight::compute_entity_weight;
use crate::core::entity::systems::apply_gravity_to_entities::apply_gravity_to_entities;
use crate::core::entity::systems::integrate_position_and_velocity::{integrate_positions, integrate_velocity};
use crate::core::entity::systems::apply_ground_constraint::apply_ground_constraint;
use crate::core::entity::systems::accumulate_exposure::accumulate_exposure;
use crate::core::entity::systems::accumulate_absorbed_energy::accumulate_absorbed_energy;
use crate::core::entity::systems::update_temperature_from_internal_energy::update_temperature_from_internal_energy;
use crate::core::entity::systems::apply_radiative_cooling::apply_radiative_cooling;
use crate::core::entity::systems::apply_absorbed_energy::apply_absorbed_energy;
use crate::core::entity::systems::update_entity_exposure_area::update_entity_exposure_area;
use crate::core::entity::systems::material::update_effective_density::update_effective_density;
use crate::core::entity::systems::material::update_effective_specific_heat::update_effective_specific_heat;
use crate::core::entity::systems::material::update_effective_emissivity::update_effective_emissivity;
use crate::core::entity::systems::material::update_effective_absorptivity::update_effective_absorptivity;


use crate::core::render::view::ViewFrame;
use crate::core::render::build_view;


#[derive(Default)]
pub struct SimulationEngine {
    pub time: SimTime,
    pub tick_delta_ns: i128,
    
    pub state: SimulationState,
}

impl SimulationEngine {
    pub fn new(
        time: SimTime,
        tick_delta_ns: i128,
        entities: EntityStore,
    ) -> Self {
        Self {
            time,
            tick_delta_ns,
            
            state: SimulationState {
                entities,
                ..Default::default()
            },
        }
    }

    pub fn new_with_state(
            time: SimTime,
            tick_delta_ns: i128,
            cosmic: CosmicState,
            world: WorldState,
            entities: EntityStore,
        ) -> Self {
            Self {
                time,
                tick_delta_ns,
               
                state: SimulationState {
                    cosmic,
                    world,
                    entities,
                    ..Default::default()
                },
            }
        }

    pub fn new_with_full_state(
        time: SimTime,
        tick_delta_ns: i128,
        cosmic: CosmicState,
        world: WorldState,
        environment: EnvironmentState,
        entities: EntityStore,
    ) -> Self {
        Self {
            time,
            tick_delta_ns,
         
            state: SimulationState {
                cosmic,
                world,
                environment,
                entities,
            },
        }
    }

    pub fn render_view(&self, view: &ViewFrame) -> Vec<RenderPrimitive> {
        build_view(view, &self.state, self.time)
    }





    pub fn step(&mut self) {
        self.time = self.time + SimDuration::from_ns(self.tick_delta_ns);

    }

   pub fn tick(&mut self) {
    self.time += SimDuration::from_ns(self.tick_delta_ns);
    let dt = Seconds(self.tick_delta_ns as f64 * 1e-9);

    //  Environment
    sample_environment_for_active_entities(
        self.time,
        &self.state.cosmic,
        &self.state.world,
        &self.state.environment,
        &mut self.state.entities,
    );

    //  Derived geometry
    update_entity_volume(&mut self.state.entities);
    update_entity_surface_area(&mut self.state.entities);
    update_entity_exposure_area(&mut self.state.entities);

    //  Materials -> effective properties
    update_effective_density(&mut self.state.entities);
    update_effective_specific_heat(&mut self.state.entities);
    update_effective_emissivity(&mut self.state.entities);
    update_effective_absorptivity(&mut self.state.entities);

    //  Derived physics
    compute_entity_mass(&mut self.state.entities);
    compute_entity_weight(&mut self.state.entities);

    //  Radiation → energy
    accumulate_exposure(&mut self.state.entities, dt);
    accumulate_absorbed_energy(&mut self.state.entities);
    apply_absorbed_energy(&mut self.state.entities);
    apply_radiative_cooling(&mut self.state.entities, dt);

    //  Energy → temperature
    update_temperature_from_internal_energy(&mut self.state.entities);

    //  Forces + motion
    apply_gravity_to_entities(&mut self.state.entities);
    integrate_velocity(&mut self.state.entities, dt);
    integrate_positions(&mut self.state.entities, dt);
    apply_ground_constraint(&mut self.state.entities, &self.state.world, &self.state.cosmic);
}



}

