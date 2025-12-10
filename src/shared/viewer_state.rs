use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use crate::core::uvoxid::{UvoxId, LatCode, LonCode, RUm};
use crate::sim::components::quaternion::QuaternionLocal;
use crate::core::id::UserId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CameraState {
    pub pos: UvoxId,
    pub orient: QuaternionLocal,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            pos: UvoxId::earth_surface(
                LatCode::from_degrees(0.0),
                LonCode::from_degrees(0.0),
            ),
            orient: QuaternionLocal::identity(),
        }
    }
}

#[derive(Default)]
pub struct ViewerRegistry {
    pub cameras: HashMap<UserId, CameraState>,
}
