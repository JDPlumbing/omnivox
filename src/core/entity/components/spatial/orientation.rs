use crate::core::math::quat::Quat;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Orientation(pub Quat);
