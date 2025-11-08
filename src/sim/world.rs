use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::supabasic::events::EventRow;
use crate::supabasic::WorldRow;
use std::collections::HashMap;
use crate::objex::core::types::{Objex};
use uuid::Uuid;
use crate::sim::components::{Velocity, Acceleration, FractureData, CorrosionData};
use crate::objex::systems::{
    degradation::DegradationProps,
    electrical::ElectricalProps,
    mechanical::MechanicalProps,
    thermal::ThermalProps,
    strength::StrengthProps,
    mass::MassProps,
    optical::OpticalProps,
};
use crate::objex::core::composite::CompositeProps;

/// Persistent world metadata (used in Supabase)
#[derive(Debug, Serialize, Deserialize)]
pub struct World {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
    /// Runtime-only: events that occurred within this world
    #[serde(default)]
    pub events: Vec<EventRow>,
    /// Runtime-only: active objects within this world
    #[serde(skip)]
    pub objects: HashMap<String, Objex>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWorld {
    pub frame_id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            frame_id: 0,
            name: Some("Test-Earth".into()),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
            events: vec![],
            objects: HashMap::new(),
        }
    }
}

/// In-memory simulation state for a running world
#[derive(Debug)]
pub struct WorldState {
    pub meta: WorldRow,                    // persisted metadata
    pub events: Vec<EventRow>,             // runtime events
    pub objects: HashMap<String, Objex>,   // active objects

    // basic physics
    pub velocity_components: HashMap<Uuid, Velocity>,
    pub acceleration_components: HashMap<Uuid, Acceleration>,

    // transient or derived components
    pub fracture_components: HashMap<Uuid, FractureData>,
    pub corrosion_components: HashMap<Uuid, CorrosionData>,

    // full physics/property systems
    pub mass_components: HashMap<Uuid, MassProps>,
    pub mechanical_components: HashMap<Uuid, MechanicalProps>,
    pub strength_components: HashMap<Uuid, StrengthProps>,
    pub thermal_components: HashMap<Uuid, ThermalProps>,
    pub electrical_components: HashMap<Uuid, ElectricalProps>,
    pub degradation_components: HashMap<Uuid, DegradationProps>,
    pub optical_components: HashMap<Uuid, OpticalProps>,
    pub composite_components: HashMap<Uuid, CompositeProps>,
}

impl WorldState {
    pub fn new(meta: WorldRow) -> Self {
        Self {
            meta,
            events: Vec::new(),
            objects: HashMap::new(),
            velocity_components: HashMap::new(),
            acceleration_components: HashMap::new(),
            fracture_components: HashMap::new(),
            corrosion_components: HashMap::new(),
            mass_components: HashMap::new(),
            mechanical_components: HashMap::new(),
            strength_components: HashMap::new(),
            thermal_components: HashMap::new(),
            electrical_components: HashMap::new(),
            degradation_components: HashMap::new(),
            optical_components: HashMap::new(),
            composite_components: HashMap::new(),
        }
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new(WorldRow {
            frame_id: 0,
            name: Some("Default World".into()),
            description: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            deleted_at: None,
        })
    }
}
