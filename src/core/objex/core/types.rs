use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::core::objex::matcat::materials::MatCatId;

/// Canonical physical object definition.
/// Geometry is referenced, not embedded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Objex {
    pub id: Uuid,

    /// Reference to canonical geometry
    pub geospec_id: Uuid,

    /// Material identity (deterministic, not UUID)
    pub matcat: MatCatId,
}

impl Objex {
    /// Create a new Objex from an existing GeoSpec
    pub fn new(geospec_id: Uuid, matcat: MatCatId) -> Self {
        Self {
            id: Uuid::new_v4(),
            geospec_id,
            matcat,
        }
    }

    /// Create with explicit ID (used by stores / DB)
    pub fn with_id(id: Uuid, geospec_id: Uuid, matcat: MatCatId) -> Self {
        Self {
            id,
            geospec_id,
            matcat,
        }
    }
}

