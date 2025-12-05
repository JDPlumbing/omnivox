use serde::{Serialize, Deserialize};
use serde_json::Value;
//use uuid::Uuid;
use crate::core::id::{WorldId, EntityId};

use crate::core::tdt::sim_time::SimTime;
use crate::core::tdt::sim_time::serialize_simtime;
use crate::supabasic::events::EventRow;

/// ---------------------------------------------------------------------------
/// ChronoEvent — a *command/event* describing what happened to an entity.
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoEvent {
    /// Which entity this event refers to
    pub entity_id: EntityId,

    /// World where the event occurred
    pub world_id: WorldId,

    /// Absolute simulation timestamp
    #[serde(serialize_with = "serialize_simtime")]
    pub t: SimTime,

    /// What happened
    pub kind: EventKind,

    /// Optional extra info
    #[serde(default)]
    pub payload: Option<Value>,
}

/// ---------------------------------------------------------------------------
/// Event categories
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    // --- Lifecycle ---
    Spawn,
    Despawn,

    // --- Motion ---
    Move { dr: i64, dlat: i64, dlon: i64 },
    Accelerate { ar: f64, alat: f64, alon: f64 },
    Teleport { r_um: i64, lat_code: i64, lon_code: i64 },

    // --- Environment ---
    TemperatureChange { delta_c: f64 },
    PressureChange { delta_pa: f64 },
    Radiation { dose: f64 },
    Shock { g: f64 },

    // --- Material ---
    Degrade { rate: f64 },
    Leak { severity: f64 },
    Fracture { plane: String },

    // --- Interactions ---
    Bond { with: EntityId },
    Unbond { from: EntityId },
    Transfer {
        to: EntityId,
        what: String,
        amount: f64,
    },

    // --- Catch-all ---
    Custom(String),
}

impl ChronoEvent {
    /// Create a new event with no payload.
    #[inline]
    pub fn new(entity_id: EntityId, world_id: WorldId, t: SimTime, kind: EventKind) -> Self {

            Self {
                entity_id,
                world_id,
                t,
                kind,
                payload: None,
            }
        }

    /// Add payload fluently
    #[inline]
    pub fn with_payload(mut self, payload: Value) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Create a simple custom event
    #[inline]
    pub fn custom(entity_id: EntityId, world_id: WorldId, t: SimTime, label: impl Into<String>) -> Self {

        ChronoEvent::new(entity_id, world_id, t, EventKind::Custom(label.into()))
    }
}

/// ---------------------------------------------------------------------------
/// Convert DB row → ChronoEvent
/// (DB stores absolute ns timestamps + serialized kind + payload)
/// ---------------------------------------------------------------------------
impl From<EventRow> for ChronoEvent {
    fn from(r: EventRow) -> Self {
        ChronoEvent {
            entity_id: r.entity_id,
            world_id: r.world_id,

            t: SimTime::from_ns(r.ticks as i128),

            kind: serde_json::from_str(&r.kind)
                .unwrap_or(EventKind::Custom(r.kind)),

            payload: r.payload,
        }
    }
}
