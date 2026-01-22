use crate::core::id::WorldId;
use crate::core::uvoxid::UvoxId;
use crate::core::tdt::SimTime;
use crate::core::observer::ObserverId;
use crate::core::world::{WorldResolver, WorldEnvironment};
use crate::core::physics::environmental_snapshot::{
    EnvironmentalSnapshot,
    sample_environmental_snapshot,
};
use crate::core::physics::tides::AnchorError;

#[derive(Debug, Clone)]
pub struct Observer {
    pub id: ObserverId,
    pub world: WorldId,
    pub uvox: UvoxId,

    /// When this observer became valid
    pub created_at: SimTime,
}

impl Observer {
    pub fn snapshot(
        &self,
        resolver: &WorldResolver,
        env: &WorldEnvironment,
        time: SimTime,
    ) -> Result<EnvironmentalSnapshot, AnchorError> {
        sample_environmental_snapshot(
            resolver,
            self.world,
            &self.uvox,
            time,
            env,
        )
    }
}
