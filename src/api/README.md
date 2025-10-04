# Omnivox API Module

This module provides the REST API layer for the Omnivox simulation backend, implemented in Rust using [Axum](https://github.com/tokio-rs/axum). It exposes endpoints for managing and querying simulations, worlds, users, objects, and events, and acts as the main interface between the frontend and the backend data models.

## Structure

- **mod.rs**: Top-level API router, wiring all endpoints and applying CORS.
- **worlds.rs**: Endpoints for listing, retrieving, and creating simulation worlds.
- **simulations.rs**: Endpoints for listing and retrieving simulation sessions.
- **events.rs**: Endpoints for listing, retrieving, and creating simulation events.
- **objex.rs**: Endpoints for listing, retrieving, and creating simulation objects ("objex").
- **users.rs**: Endpoints for retrieving users (both authenticated and anonymous).

## Key Endpoints

- `GET /worlds` — List all worlds, including their events.
- `GET /worlds/{id}` — Retrieve a single world by frame ID, including its events.
- `POST /worlds` — Create a new world.
- `GET /simulations` — List all simulations.
- `GET /simulations/{id}` — Retrieve a simulation by ID, including its events.
- `GET /events` — List all events.
- `GET /events/{id}` — Retrieve a single event.
- `POST /events` — Create a new event.
- `GET /objex` — List all objects.
- `GET /objex/{id}` — Retrieve a single object.
- `POST /simulations/{sim_id}/objex` — Create a new object in a simulation.
- `GET /users/{id}` — Retrieve a user by ID.
- `GET /anon_users` — List all anonymous users.
- `POST /anon_users` — Create a new anonymous user.
- `GET /anon_users/{id}` — Retrieve an anonymous user by ID.

## Data Flow

- All endpoints interact with the Supabase backend via the `Supabase` client abstraction.
- DTOs (Data Transfer Objects) are defined for each resource to ensure consistent API responses.
- Events are hydrated into worlds and simulations where relevant.

## Conventions

- All API responses are JSON.
- Errors are returned with appropriate HTTP status codes and error messages.
- DTOs are constructed via `From` implementations for clean separation between database models and API responses.

## Extending

- To add a new resource, create a new file (e.g., `widgets.rs`), define the DTO and handlers, and wire it up in `mod.rs`.
- Follow the existing pattern for error handling and DTO conversion.

## See Also

- [src/supabasic/](../supabasic/) — Database models and Supabase client logic.
- [src/sim/](../sim/) — Core simulation logic and domain models.

---

**Maintainer:** drippy  
**License:** MIT
