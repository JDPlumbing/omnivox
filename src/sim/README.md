# sim Module

The `sim` module provides the core simulation engine for Omnivox. It manages simulation state, world data, event timelines, and the execution of modular systems (such as movement, physics, or custom logic) over time. This module is responsible for loading, running, and persisting simulation sessions, and for orchestrating the flow of events and state changes in a simulated world.

## Features

- **Simulation Orchestration:**  
  - `Simulation` struct manages the simulation loop, tick advancement, and system execution.
  - Modular `System` trait allows plugging in custom logic (e.g., movement, physics, AI).
- **World State Management:**  
  - `World` struct holds the current state, metadata, and associated events for a simulation frame.
- **Event Timeline:**  
  - Uses `ChronoEvent` and a timeline vector to record and replay all state changes.
- **Persistence:**  
  - Async loading and saving of simulation state, worlds, and events via Supabase.
  - Helpers for spawning entities and persisting both objects and events.
- **Error Handling:**  
  - Unified `OmnivoxError` type for all simulation-related errors.

## Structure

- **mod.rs:** Module root and exports.
- **simulation.rs:** Defines the `Simulation` struct, tick logic, and system execution.
- **world.rs:** Defines the `World` struct and new world creation.
- **systems/**: Modular systems implementing the `System` trait (e.g., movement).
- **load.rs:** Async loading of simulation state and timelines from Supabase.
- **persist.rs:** Async persistence helpers for objects, events, and entity spawning.
- **error.rs:** Error types and result alias.

## Example Usage

```rust
use sim::{Simulation, World, systems::movement::MovementSystem};

// Create a world and simulation
let world = World { /* ... */ };
let systems: Vec<Box<dyn sim::systems::System + Send>> = vec![Box::new(MovementSystem)];
let mut sim = Simulation::new(world, systems);

// Advance the simulation by one tick
let events = sim.tick();
println!("Events this tick: {:?}", events);