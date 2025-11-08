use crate::chronovox::{UvoxId, TimeDelta, Cartesian};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::supabasic::events::EventRow;

/// A Chronovox event: something happening at a place + time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChronoEvent {
    /// Where it occurred (spatial ID).
    pub id: UvoxId,
    /// When it occurred (time delta from epoch).
    pub t: TimeDelta,
    /// What happened.
    pub kind: EventKind,
    /// Optional extra data (system-specific).
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

/// Defines the types of events Chronovox can represent.
/// 
/// Each variant is a *class of change*, not a hyper-specific action.
/// Systems logic interprets these depending on context (material, env, etc).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventKind {
    // === Core Lifecycle ===
    /// An object/entity comes into existence.
    Spawn,
    /// An object/entity ceases to exist.
    Despawn,

    // === Movement & Position ===
    /// Object moves relative to its current position.
    Move { dr: i64, dlat: i64, dlon: i64 },
    Accelerate { ar: f64, alat: f64, alon: f64 },

    /// Object jumps to a new absolute position.
    Teleport { r_um: u64, lat_code: i64, lon_code: i64 },

    // === Environment Effects ===
    /// Change in temperature (°C).
    TemperatureChange { delta_c: f64 },
    /// Change in pressure (Pascals).
    PressureChange { delta_pa: f64 },
    /// Radiation exposure (dose in Sieverts).
    Radiation { dose: f64 },
    /// Sudden impact/shock (acceleration in g).
    Shock { g: f64 },

    // === Material / Integrity ===
    /// Generic degradation over time.
    Degrade { rate: f64 },
    /// Fluid/gas leakage.
    Leak { severity: f64 },
    /// Structural fracture.
    Fracture { plane: String },

    // === Interactions ===
    /// Form a bond with another entity.
    Bond { with: Uuid },
    /// Break an existing bond.
    Unbond { from: Uuid },
    /// Transfer something between entities.
    Transfer {
        to: Uuid,
        what: String,
        amount: f64,
    },

    // === Wild Card ===
    /// Catch-all for events not yet modeled in the vocabulary.
    Custom(String),
}

impl ChronoEvent {
    /// A simple placeholder for testing
    pub fn dummy() -> Self {
        Self {
            id: UvoxId {
                frame_id: 1,
                r_um: 0,
                lat_code: 0,
                lon_code: 0,
            },
            t: TimeDelta::from_now(),
            kind: EventKind::Spawn,
            payload: None,
        }
    }

    pub fn new() -> Self {
        Self {
            id: UvoxId::new(0, 0, 0, 0),
            t: TimeDelta::from_ticks(0, "nanoseconds"),
            kind: EventKind::Custom("Undefined".into()),
            payload: None,
        }
    }
}


impl From<EventRow> for ChronoEvent {
    fn from(r: EventRow) -> Self {
        ChronoEvent {
            id: UvoxId::new(r.frame_id, r.r_um, r.lat_code, r.lon_code),
            t: TimeDelta::from_ticks(r.ticks, "nanoseconds"),
            kind: serde_json::from_str(&r.kind).unwrap_or(EventKind::Custom(r.kind)),
            payload: r.payload,
        }
    }
}
