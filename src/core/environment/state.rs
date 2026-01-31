use std::collections::HashMap;
use crate::core::worlds::id::WorldId;
use crate::core::environment::components::{
    atmosphere::AtmosphereDescriptor,
    composition::AtmosphericComposition,
};

#[derive(Default)]
pub struct EnvironmentState {
    pub atmospheres: HashMap<WorldId, AtmosphereDescriptor>,
    pub compositions: HashMap<WorldId, AtmosphericComposition>,
}

