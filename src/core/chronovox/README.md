# Chronovox Module

The **Chronovox** module provides the core event sourcing, timeline, and state playback engine for the Omnivox simulation system. It models all changes in the simulation as time-stamped events, enabling robust replay, querying, and state reconstruction for any entity.

## Features

- **Event Sourcing:** All changes (spawns, moves, environment effects, interactions, etc.) are represented as `ChronoEvent` records.
- **Timeline Playback:** The `Timeline` struct can replay events to reconstruct the state of any entity at any point in time.
- **Persistence:** Events are stored and retrieved from the database (via Supabase) using the `persist` module.
- **Extensible Event Vocabulary:** The `EventKind` enum covers core simulation actions and can be extended for new types of events.
- **Error Handling:** Unified error types via `ChronovoxError` for consistent error management across the module.

## Structure

- **mod.rs:** Module exports and re-exports for easy integration.
- **event.rs:** Defines the `ChronoEvent` struct and the `EventKind` enum, which describe all possible simulation events.
- **timeline.rs:** Implements the `Timeline` struct for event playback, state reconstruction, and time-based queries.
- **persist.rs:** Handles database persistence and retrieval of events for entities.
- **error.rs:** Defines the `ChronovoxError` enum and unified `Result` type for error handling.

## Key Types

- `ChronoEvent`: Represents a single event in the simulation (with time, location, kind, and optional payload).
- `EventKind`: Enumerates all supported event types (spawn, move, temperature change, bond, etc.).
- `Timeline`: A sequence of `ChronoEvent`s with methods for playback and querying.
- `EntityState`: Represents the reconstructed state of an entity after applying events.
- `ChronovoxError`: Unified error type for all Chronovox operations.

## Usage

- **Insert an event:**  
  Use `insert_event_for_entity(supa, entity_id, &event)` to persist a new event for an entity.
- **Fetch events:**  
  Use `fetch_events_for_entity(supa, entity_id)` to retrieve all events for an entity as a `Timeline`.
- **Replay state:**  
  Use `Timeline::playback()` or `Timeline::playback_until(cutoff_ns)` to reconstruct entity state at any time.

## Example

```rust
use crate::chronovox::{ChronoEvent, EventKind, Timeline, insert_event_for_entity, fetch_events_for_entity};

// Create and insert an event
let event = ChronoEvent::dummy();
let event_id = insert_event_for_entity(&supa, entity_id, &event).await?;

// Fetch and replay timeline
let timeline = fetch_events_for_entity(&supa, entity_id).await?;
let state = timeline.playback();