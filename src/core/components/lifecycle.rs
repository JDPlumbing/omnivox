use crate::core::tdt::SimTime;
/// -------------------------------------------------------------------
/// Lifecycle component for entities
/// -------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Lifecycle {
    pub spawned_at: SimTime,
    pub despawned_at: Option<SimTime>,
}
