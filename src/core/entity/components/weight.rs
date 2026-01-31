use crate::core::physics::units::force::Newtons;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Weight(pub Newtons);
