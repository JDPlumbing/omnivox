use serde::{Deserialize, Serialize};
use crate::core::tdt::sim_time::SimTime;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Time {
    pub sim_time: SimTime,
}
