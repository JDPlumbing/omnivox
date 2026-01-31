use crate::core::cosmic::state::CosmicState;
use crate::core::worlds::state::WorldState;
use crate::core::environment::state::EnvironmentState;
use crate::shared::entities::entity_store::EntityStore;

#[derive(Default)]
pub struct SimulationState {
    pub cosmic: CosmicState,
    pub world: WorldState,
    pub environment: EnvironmentState,
    pub entities: EntityStore,
}
