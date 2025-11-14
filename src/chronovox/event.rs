use crate::chronovox::UvoxId;
use crate::tdt::sim_time::SimTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::supabasic::events::EventRow;

/// A Chronovox event: something happening at a place + time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoEvent {
    /// Where it occurred (spatial ID).
    pub id: UvoxId,

    /// Absolute simulation timestamp (ns since Unix epoch).
    pub t: SimTime,

    /// What happened.
    pub kind: EventKind,

    /// Optional extra data.
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
    pub fn new(id: UvoxId, t: SimTime, kind: EventKind) -> Self {
        Self { id, t, kind, payload: None }
    }

    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }
}

/// Convert DB row → ChronoEvent
impl From<EventRow> for ChronoEvent {
    fn from(r: EventRow) -> Self {
        ChronoEvent {
            id: UvoxId::new(r.frame_id, r.r_um, r.lat_code, r.lon_code),
            t: SimTime::from_ns(r.ticks as i128), // DB already stores ns
            kind: serde_json::from_str(&r.kind).unwrap_or(EventKind::Custom(r.kind)),
            payload: r.payload,
        }
    }
}
