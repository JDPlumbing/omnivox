use crate::core::chronovox::UvoxId;
use crate::core::tdt::sim_time::SimTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::supabasic::events::EventRow;
use chrono::SecondsFormat;

/// A Chronovox event: something happening at a place + time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoEvent {
    /// Where it occurred (spatial ID)
    pub id: UvoxId,

    /// Absolute simulation timestamp (ns since Unix epoch)
    #[serde(serialize_with = "crate::core::tdt::sim_time::serialize_simtime")]

    pub t: SimTime,

    /// What happened
    pub kind: EventKind,
    
    /// Optional extra data
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

/// Types of events Chronovox can represent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    // --- Lifecycle ---
    Spawn,
    Despawn,

    // --- Movement ---
    Move { dr: i64, dlat: i64, dlon: i64 },
    Accelerate { ar: f64, alat: f64, alon: f64 },
    Teleport { r_um: u64, lat_code: i64, lon_code: i64 },

    // --- Environment ---
    TemperatureChange { delta_c: f64 },
    PressureChange { delta_pa: f64 },
    Radiation { dose: f64 },
    Shock { g: f64 },

    // --- Materials ---
    Degrade { rate: f64 },
    Leak { severity: f64 },
    Fracture { plane: String },

    // --- Interactions ---
    Bond { with: Uuid },
    Unbond { from: Uuid },
    Transfer {
        to: Uuid,
        what: String,
        amount: f64,
    },

    // --- Catch-all ---
    Custom(String),
}

impl ChronoEvent {
    /// Create a bare event with no payload
    pub fn new(id: UvoxId, t: SimTime, kind: EventKind) -> Self {
        Self { id, t, kind, payload: None }
    }

    /// Create an event at the world’s current SimTime
    pub fn at_now(id: UvoxId, world: &crate::sim::world::WorldState, kind: EventKind) -> Self {
        let t = world.clock
            .as_ref()
            .expect("world.clock missing")
            .current;

        Self { id, t, kind, payload: None }
    }

    /// Add payload fluently
    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }

    /// Custom event with a freeform string kind
    pub fn custom(id: UvoxId, t: SimTime, label: impl Into<String>) -> Self {
        ChronoEvent::new(id, t, EventKind::Custom(label.into()))
    }
}

/// Convert DB row → ChronoEvent
/// Assumes DB stores absolute nanoseconds since Unix epoch.
impl From<EventRow> for ChronoEvent {
    fn from(r: EventRow) -> Self {
        ChronoEvent {
            id: UvoxId::new(r.frame_id, r.r_um, r.lat_code, r.lon_code),

            // DB column already contains whole ns
            t: SimTime::from_ns(r.ticks as i128),

            kind: serde_json::from_str(&r.kind)
                .unwrap_or(EventKind::Custom(r.kind)),

            payload: r.payload,
        }
    }
}
