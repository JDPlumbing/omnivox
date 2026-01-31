// core/worlds/state.rs

use std::collections::HashMap;

use crate::core::worlds::id::WorldId;
use crate::core::worlds::components::{
        world_anchor::WorldAnchor,
        world_orientation::WorldOrientation,
        world_surface::WorldSurface,
};

#[derive(Default)]
pub struct WorldState {
    pub anchors: HashMap<WorldId, WorldAnchor>,
    pub orientations: HashMap<WorldId, WorldOrientation>,
    pub surfaces: HashMap<WorldId, WorldSurface>,
}

