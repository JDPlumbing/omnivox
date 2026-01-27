use serde::Deserialize;
use crate::core::uvoxid::UvoxId;

#[derive(Debug, Deserialize)]
pub struct SetPositionDto {
    pub uvox: UvoxId,
}
