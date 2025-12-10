use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CameraDelta {
    pub yaw: f32,           // radians
    pub pitch: f32,         // radians
    pub roll: f32,          // radians
    pub move_forward: f32,  // meters per tick
    pub move_right: f32,    // meters per tick
    pub move_up: f32,       // meters per tick
}

