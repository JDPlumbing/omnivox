# sim::world

This package contains the in-memory world model and helpers used by the simulation runtime. It provides a small domain-layer model for a simulation "world" and a loader that hydrates that model from the Supabase-backed database rows (via the `supabasic` client).

The code in this folder is intentionally lightweight: domain metadata (the `World` struct) is separated from persisted DB records (`WorldRecord`) and the runtime state (`WorldState`) is optimized for fast access by systems and tick logic.

## Purpose

- Represent domain-level world metadata and the running world state.
- Convert database records into simulation runtime objects (`SimEntity`).
- Provide a single entry-point loader (`load_world`) that returns a fully populated `WorldState` ready for simulation.

## Key files

- `state.rs` — Domain metadata + in-memory runtime state:
  - `World` — Domain-level metadata about a world (not persisted here). Fields:
    - `world_id: i64`
    - `name: Option<String>`
    - `description: Option<String>`
    - `world_epoch: Option<SimTime>`
  - `WorldState` — Runtime state used while a world is running:
    - `meta: World`
    - `entities: HashMap<Uuid, SimEntity>`
    - `events: Vec<EventRow>` (in-memory event buffer)
    - `sim_time: SimTime`
    - `sim_delta: SimDuration`
    - `clock: Option<SimClock>`
    - `components: SimComponents`
  - `From<WorldRecord> for World` — convert DB world record → domain `World`.

- `loader.rs` — Async loader that builds a `WorldState`:
  - `pub async fn load_world(supa: &Supabase, world_id: i64) -> Result<WorldState>`  
    Steps:
      1. Fetch the `WorldRecord` via `WorldRecord::fetch`.
      2. Fetch entity rows for the world via `EntityRecord::list_for_world`.
      3. Convert each `EntityRecord` into `SimEntity` using `TryFrom` / `try_into`.
      4. Build and return `WorldState::from_entities(meta, entities)`.

## How to use

Example code to load a world from within an async context (Tokio):

```rust
use crate::supabasic::Supabase;
use crate::sim::world::loader::load_world;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create or get your Supabase client (configuration per your app)
    let supa = Supabase::new_from_env()?; // or whatever constructor you use

    let world_id = 42;
    let world_state = load_world(&supa, world_id).await?;

    println!("Loaded world: {:?}", world_state.meta.name);
    println!("Entity count: {}", world_state.entities.len());

    Ok(())
}