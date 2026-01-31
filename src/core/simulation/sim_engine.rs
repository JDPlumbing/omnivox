use crate::shared::entities::entity_store::EntityStore;
use crate::core::physics::units::{time::Seconds,
                                        };

use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_duration::SimDuration;

use crate::core::cosmic::state::CosmicState;

use crate::core::worlds::state::WorldState;

use crate::core::environment::systems::sampling::sample_environment_for_active_entities;
use crate::core::entity::systems::mass::compute_entity_mass;
use crate::core::entity::systems::weight::compute_entity_weight;
use crate::core::entity::systems::gravity::apply_gravity_to_entities;
use crate::core::entity::systems::movement::{integrate_positions, integrate_velocity};
use crate::core::entity::systems::ground_constraint::apply_ground_constraint;
use crate::core::entity::systems::exposure::accumulate_exposure;
use crate::core::entity::systems::accumulate_absorbed_energy::accumulate_absorbed_energy;
use crate::core::entity::systems::temperature::update_temperature_from_internal_energy;
use crate::core::entity::systems::radiative_cooling::apply_radiative_cooling;
use crate::core::entity::systems::apply_absorbed_energy::apply_absorbed_energy;
use crate::core::simulation::state::SimulationState;



use crate::core::render::view::ViewFrame;


#[derive(Default)]
pub struct SimulationEngine {
    pub time: SimTime,
    pub tick_delta_ns: i128,
    pub view: ViewFrame,
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
            view: ViewFrame::default(),
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
                view: ViewFrame::default(),
                state: SimulationState {
                    cosmic,
                    world,
                    entities,
                    ..Default::default()
                },
            }
        }
    

    pub fn step(&mut self) {
        self.time = self.time + SimDuration::from_ns(self.tick_delta_ns);

    }
pub fn tick(&mut self) {
    // 1. Advance time
    self.time += SimDuration::from_ns(self.tick_delta_ns);
    let dt = Seconds(self.tick_delta_ns as f64 * 1e-9);

    // 2. Sample environment
    sample_environment_for_active_entities(
        self.time,
        &self.state.cosmic,
        &self.state.world,
        &self.state.environment,
        &mut self.state.entities,
    );

    // 3. Derived physical properties
    compute_entity_mass(&mut self.state.entities);
    compute_entity_weight(&mut self.state.entities);

    // 4. Radiation → energy
    accumulate_exposure(&mut self.state.entities, dt);
    accumulate_absorbed_energy(&mut self.state.entities);
    apply_absorbed_energy(&mut self.state.entities); // 🔑 MOVE THIS UP
    apply_radiative_cooling(&mut self.state.entities, dt);
    // 5. Energy → temperature
    update_temperature_from_internal_energy(&mut self.state.entities);

    // 6. Temperature → radiative cooling


    // 7. Forces
    apply_gravity_to_entities(&mut self.state.entities);

    // 8. Integrate motion
    integrate_velocity(&mut self.state.entities, dt);
    integrate_positions(&mut self.state.entities, dt);

    // 9. Constraints
    apply_ground_constraint(
        &mut self.state.entities,
        &self.state.world,
    );
}

}

