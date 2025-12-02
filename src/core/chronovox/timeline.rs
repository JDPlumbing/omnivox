use std::cmp::Ordering;

use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::core::chronovox::event::{ChronoEvent};
use crate::core::id::EntityId;

/// ---------------------------------------------------------------------------
/// Timeline — a simple ordered log of ChronoEvents
/// ---------------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timeline {
    pub events: Vec<ChronoEvent>,
}

impl Timeline {
    /// Create an empty timeline
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    /// Push without sorting (for bulk inserts)
    pub fn push(&mut self, event: ChronoEvent) {
        self.events.push(event);
    }

    /// Insert and maintain chronological ordering
    pub fn insert(&mut self, event: ChronoEvent) {
        self.events.push(event);
        self.events.sort(); // relies on Ord for ChronoEvent
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
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

    /// Query events for a specific entity
    pub fn query_entity(&self, entity_id: EntityId) -> Vec<&ChronoEvent> {
        self.events
            .iter()
            .filter(|e| e.entity_id == entity_id)
            .collect()
    }
}

/// ---------------------------------------------------------------------------
/// Sorting ChronoEvents chronologically
/// ---------------------------------------------------------------------------
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

/// Iterators
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
