use std::cmp::Ordering;
use std::collections::HashMap;
use crate::chronovox::{ChronoEvent, EventKind, UvoxId, Cartesian};

#[derive(Debug, Default, Clone)]
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
        self.events.sort(); // keep it ordered
    }

    pub fn iter_chronological(&self) -> impl Iterator<Item = &ChronoEvent> {
        self.events.iter()
    }

    pub fn query_time_range(&self, start_ns: i64, end_ns: i64) -> Vec<&ChronoEvent> {
        self.events
            .iter()
            .filter(|e| {
                let t = e.t.ticks("nanoseconds");
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
                // === Core Lifecycle ===
                EventKind::Spawn => {
                    state.insert(
                        e.id,
                        EntityState {
                            r_um: 6_731_000_000, // approx Earth's radius in um
                            lat_code: 0,
                            lon_code: 0,
                            alive: true,
                            temperature: 20.0, // default room temp
                            pressure: 101_325.0, // default 1 atm
                        },
                    );
                }

                EventKind::Despawn => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.alive = false;
                    }
                }

                // === Movement ===
                EventKind::Move { dr, dlat, dlon } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = ((s.r_um as i64) + dr).max(0) as u64;
                        s.lat_code += dlat;
                        s.lon_code += dlon;
                    }
                }


                EventKind::Teleport { r_um, lat_code, lon_code } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = *r_um;
                        s.lat_code = *lat_code;
                        s.lon_code = *lon_code;
                    }
                }


                // === Environment ===
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

                EventKind::Radiation { dose: _ } => {
                    // TODO
                }
                EventKind::Shock { g: _ } => {
                    // TODO
                }

                // === Material / Integrity ===
                EventKind::Degrade { rate: _ } => {
                    // TODO
                }
                EventKind::Leak { severity: _ } => {
                    // TODO
                }
                EventKind::Fracture { plane: _ } => {
                    // TODO
                }

                // === Interactions ===
                EventKind::Bond { with: _ } => {
                    // TODO: mark bonded state
                }
                EventKind::Unbond { from: _ } => {
                    // TODO: mark bond broken
                }
                EventKind::Transfer { to: _, what: _, amount: _ } => {
                    // TODO: transfer logic
                }

                // === Wild Card ===
                EventKind::Custom(_) => {
                    // maybe log it or trigger hooks
                }
            }
        }
        state
    }

    
    /// Reconstruct state up to a given time (with interpolation for Move)
    pub fn playback_until(&self, cutoff_ns: i64) -> HashMap<UvoxId, EntityState> {
        let mut state: HashMap<UvoxId, EntityState> = HashMap::new();
        let mut last_event_by_id: HashMap<UvoxId, &ChronoEvent> = HashMap::new();

        for e in self.iter_chronological() {
            let t = e.t.ticks("nanoseconds");

            if t > cutoff_ns {
                // Handle interpolation between two Move events
                if let Some(prev) = last_event_by_id.get(&e.id)
                    && let (EventKind::Move { dr: prev_dr, dlat: prev_dlat, dlon: prev_dlon },
                            EventKind::Move { dr: next_dr, dlat: next_dlat, dlon: next_dlon }) = (&prev.kind, &e.kind)
                {
                    let t_prev = prev.t.ticks("nanoseconds");
                    let t_next = t;
                    let frac = (cutoff_ns - t_prev) as f64 / (t_next - t_prev) as f64;

                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = (s.r_um as i64 + ((*prev_dr + *next_dr) as f64 * frac) as i64).max(0) as u64;
                        s.lat_code += ((*prev_dlat + *next_dlat) as f64 * frac) as i64;
                        s.lon_code += ((*prev_dlon + *next_dlon) as f64 * frac) as i64;
                    }
                }

                break; // stop at cutoff
            }

            // Normal event application
            match &e.kind {
                // === Core Lifecycle ===
                EventKind::Spawn => {
                    state.insert(
                        e.id,
                        EntityState {
                            r_um: 6_731_000_000, // approx Earth's radius in um
                            lat_code: 0,
                            lon_code: 0,
                            alive: true,
                            temperature: 20.0,   // default °C
                            pressure: 101_325.0, // default Pa
                        },
                    );
                }
                EventKind::Despawn => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.alive = false;
                    }
                }

                // === Movement ===
                EventKind::Move { dr, dlat, dlon } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        // apply spherical deltas instead of Cartesian
                        s.r_um = (s.r_um as i64 + dr).max(0) as u64; // safe cast, avoid underflow
                        s.lat_code += dlat;
                        s.lon_code += dlon;
                    }
                }
                EventKind::Teleport { r_um, lat_code, lon_code } => {
                    if let Some(s) = state.get_mut(&e.id) {
                        s.r_um = *r_um;
                        s.lat_code = *lat_code;
                        s.lon_code = *lon_code;
                    }
                }


                // === Environment ===
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

                EventKind::Radiation { dose: _ } => {
                    // TODO: accumulate radiation dose
                }
                EventKind::Shock { g: _ } => {
                    // TODO: apply shock/damage
                }

                // === Material / Integrity ===
                EventKind::Degrade { rate: _ } => {
                    // TODO: mark progressive degradation
                }
                EventKind::Leak { severity: _ } => {
                    // TODO: track fluid/gas loss
                }
                EventKind::Fracture { plane: _ } => {
                    // TODO: mark fracture in state
                }

                // === Interactions ===
                EventKind::Bond { with: _ } => {
                    // TODO: link entities
                }
                EventKind::Unbond { from: _ } => {
                    // TODO: unlink entities
                }
                EventKind::Transfer { to: _, what: _, amount: _ } => {
                    // TODO: handle resource transfer
                }

                // === Wild Card ===
                EventKind::Custom(_) => {
                    // TODO: maybe just record it
                }
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

// ===== Trait Implementations =====

impl PartialEq for ChronoEvent {
    fn eq(&self, other: &Self) -> bool {
        self.t.ticks("nanoseconds") == other.t.ticks("nanoseconds")
    }
}
impl Eq for ChronoEvent {}

impl PartialOrd for ChronoEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.t.ticks("nanoseconds").cmp(&other.t.ticks("nanoseconds")))
    }
}
impl Ord for ChronoEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.ticks("nanoseconds").cmp(&other.t.ticks("nanoseconds"))
    }
}

// ===== Iterators =====

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
