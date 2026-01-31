use crate::core::cosmic::id::CosmicBodyId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldAnchor {
    pub body: CosmicBodyId,
}
