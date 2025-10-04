# supabasic Module

The `supabasic` module provides a lightweight, async client and data access layer for interacting with a Supabase backend in Rust. It abstracts RESTful CRUD operations for your simulation's core entities (users, worlds, simulations, events, objects, etc.) and provides convenient, type-safe methods for querying and manipulating data.

## Features

- **Supabase Client:**  
  - Simple, async HTTP client for Supabase REST API.
  - Supports select, insert, update, and delete operations with flexible query building.
  - Loads configuration from environment variables (`SUPABASE_URL`, `SUPABASE_KEY`).

- **ORM-like Traits and Utilities:**  
  - `DbModel` trait for table mapping and generic CRUD helpers.
  - Generic functions for fetching, listing, inserting, updating, and deleting rows.

- **Entity Modules:**  
  - `entities.rs`: Core entity records and helpers.
  - `users.rs`: User and anonymous user models and endpoints.
  - `worlds.rs`: World table mapping and CRUD.
  - `simulations.rs`: Simulation session mapping and CRUD.
  - `events.rs`: Event table mapping and helpers.
  - `objex.rs`: Object/entity mapping and conversion.

- **Error Handling:**  
  - Unified error type (`SupabasicError`) for HTTP, JSON, and custom errors.

## Structure

- **client.rs:** Supabase client and query builder.
- **orm.rs:** Generic CRUD helpers and the `DbModel` trait.
- **entities.rs:** Entity model and helpers.
- **users.rs:** User and anonymous user models and endpoints.
- **worlds.rs:** World model and helpers.
- **simulations.rs:** Simulation model and helpers.
- **events.rs:** Event model and helpers.
- **objex.rs:** Object/entity model and conversion.
- **error.rs:** Error types and result alias.

## Example Usage

```rust
use supabasic::{Supabase, Entity, WorldRow};

// Initialize client from environment
let supa = Supabase::new_from_env()?;

// Create a new entity
let entity_id = supa.create_entity("My Entity").await?;

// Fetch all worlds
let worlds = WorldRow::list(&supa).await?;

// Insert a new world
let new_world = NewWorld { frame_id: 42, name: Some("Earth".into()), description: None };
let world = WorldRow::create(&supa, &new_world).await?;