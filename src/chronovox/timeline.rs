use std::cmp::Ordering;
use std::collections::HashMap;
use crate::chronovox::{ChronoEvent, EventKind, UvoxId};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub events: Vec<ChronoEvent>,
}

#[derive(Debug, Clone)]
pub struct EntityState {
    pub r_um: u64,
    pub lat_code: i64,
    pub lon_code: i64,
    pub alive: bool,
    pub temperature: f64,
    pub pressure: f64,
}

impl Timeline {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn push(&mut self, event: ChronoEvent) {
        self.events.push(event);
    }

    pub fn insert(&mut self, event: ChronoEvent) {
        self.events.push(event);
        self.events.sort();
    }

    pub fn iter_chronological(&self) -> impl Iterator<Item = &ChronoEvent> {
        self.events.iter()
    }

    /// Query events inside a time range (ns)
    pub fn query_time_range(&self, start_ns: i128, end_ns: i128) -> Vec<&ChronoEvent> {
        self.events
            .iter()
            .filter(|e| {
                let t = e.t.as_ns();
                t >= start_ns && t <= end_ns
            })
            .collect()
    }

    pub fn query_by_id(&self, id: &UvoxId) -> Vec<&ChronoEvent> {
        self.events.iter().filter(|e| &e.id == id).collect()
    }

    pub fn playback(&self) -> HashMap<UvoxId, EntityState> {
        let mut state = HashMap::new();

        for e in self.iter_chronological() {
            match &e.kind {
                EventKind::Spawn => {
                    state.insert(
                        e.id,
                        EntityState {
                            r_um: 6_731_000_000,
                            lat_code: 0,
                            lon_code: 0,
                            alive: true,
                            temperature: 20.0,
                            pressure: 101_325.0,
                        },
                    );
                }

                EventKind::Despawn => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.alive = false;
                    }
                }

                EventKind::Move { dr, dlat, dlon } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = ((s.r_um as i64) + dr).max(0) as u64;
                        s.lat_code += dlat;
                        s.lon_code += dlon;
                    }
                }

                &EventKind::Accelerate { ar, alat, alon } => {
                    tracing::debug!("⚡ Accelerating: Δr={} Δlat={} Δlon={}", ar, alat, alon);
                }

                EventKind::Teleport { r_um, lat_code, lon_code } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = *r_um;
                        s.lat_code = *lat_code;
                        s.lon_code = *lon_code;
                    }
                }

                EventKind::TemperatureChange { delta_c } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.temperature += delta_c;
                    }
                }

                EventKind::PressureChange { delta_pa } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.pressure += delta_pa;
                    }
                }

                _ => {}
            }
        }

        state
    }

    /// State as of an absolute time, with interpolation for Move events.
    pub fn playback_until(&self, cutoff_ns: i128) -> HashMap<UvoxId, EntityState> {
        let mut state: HashMap<UvoxId, EntityState> = HashMap::new();
        let mut last_event_by_id: HashMap<UvoxId, &ChronoEvent> = HashMap::new();

        for e in self.iter_chronological() {
            let t = e.t.as_ns();

            if t > cutoff_ns {
                if let Some(prev) = last_event_by_id.get(&e.id)
                    && let (
                        EventKind::Move { dr: prev_dr, dlat: prev_dlat, dlon: prev_dlon },
                        EventKind::Move { dr: next_dr, dlat: next_dlat, dlon: next_dlon },
                    ) = (&prev.kind, &e.kind)
                {
                    let t_prev = prev.t.as_ns();
                    let t_next = t;

                    let dt_prev = (cutoff_ns - t_prev) as f64;
                    let dt_total = (t_next - t_prev) as f64;
                    let frac = dt_prev / dt_total;

                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = (s.r_um as i64 +
                            ((*prev_dr + *next_dr) as f64 * frac) as i64
                        ).max(0) as u64;

                        s.lat_code += ((*prev_dlat + *next_dlat) as f64 * frac) as i64;
                        s.lon_code += ((*prev_dlon + *next_dlon) as f64 * frac) as i64;
                    }
                }

                break;
            }

            match &e.kind {
                EventKind::Spawn => {
                    state.insert(
                        e.id,
                        EntityState {
                            r_um: 6_731_000_000,
                            lat_code: 0,
                            lon_code: 0,
                            alive: true,
                            temperature: 20.0,
                            pressure: 101_325.0,
                        },
                    );
                }

                EventKind::Despawn => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.alive = false;
                    }
                }

                EventKind::Move { dr, dlat, dlon } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = (s.r_um as i64 + dr).max(0) as u64;
                        s.lat_code += dlat;
                        s.lon_code += dlon;
                    }
                }

                &EventKind::Accelerate { ar, alat, alon } => {
                    tracing::debug!("⚡ Accelerating: Δr={} Δlat={} Δlon={}", ar, alat, alon);
                }

                EventKind::Teleport { r_um, lat_code, lon_code } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = *r_um;
                        s.lat_code = *lat_code;
                        s.lon_code = *lon_code;
                    }
                }

                EventKind::TemperatureChange { delta_c } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.temperature += delta_c;
                    }
                }

                EventKind::PressureChange { delta_pa } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.pressure += delta_pa;
                    }
                }

                _ => {}
            }

            last_event_by_id.insert(e.id, e);
        }

        state
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

// ========= Sorting impls (updated for SimTime.as_ns()) ==========

impl PartialEq for ChronoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.t.as_ns() == other.t.as_ns()
    }
}

impl Eq for ChronoEvent {}

impl PartialOrd for ChronoEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.t.as_ns().cmp(&other.t.as_ns()))
    }
}

impl Ord for ChronoEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.as_ns().cmp(&other.t.as_ns())
    }
}

impl IntoIterator for Timeline {
    type Item = ChronoEvent;
    type IntoIter = std::vec::IntoIter<ChronoEvent>;
    fn into_iter(self) -> Self::IntoIter {
        self.events.into_iter()
    }
}

impl<'a> IntoIterator for &'a Timeline {
    type Item = &'a ChronoEvent;
    type IntoIter = std::slice::Iter<'a, ChronoEvent>;
    fn into_iter(self) -> Self::IntoIter {
        self.events.iter()
    }
}
