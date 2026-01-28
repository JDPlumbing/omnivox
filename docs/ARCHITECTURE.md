# Omnivox Backend Architecture

This document describes the current backend architecture of Omnivox after the core refactor.

The primary goal of this architecture is **separation of concerns**, **offline capability**, and **infra swap-ability**.

---

## Design Goals

* Eliminate tech debt from mixed OOP / infra-driven design
* Make core simulation and domain logic runnable:

  * without HTTP
  * without a database
  * without Supabase
  * without a frontend
* Treat persistence and auth as *optional adapters*
* Allow the system to run headless, offline, and deterministically
* Make API a thin UI layer over engines

---

## High-Level Mental Model

| Concept  | Analogy                        |
| -------- | ------------------------------ |
| AppState | React Context Provider         |
| Engines  | React Hooks                    |
| Sources  | Ports (Hexagonal Architecture) |
| Infra    | Adapters                       |
| API      | UI Layer                       |

---

## Crate / Module Layout

```
src/
├── core/        # Pure domain + simulation logic (NO IO)
├── engine/      # Behavior & orchestration
├── shared/      # Traits, contexts, ports
├── infra/       # Adapters (JSON, in-memory, Supabase later)
├── app/         # Bootstrap / wiring
├── api/         # HTTP layer (Axum)
├── bin/         # Binaries (server)
```

---

## Dependency Rules (Strict)

Dependencies are **one-directional**:

```
core
 ↑
engine
 ↑
shared
 ↑
infra
 ↑
app
 ↑
api
```

Rules:

* `core` depends on nothing
* `engine` depends only on `core`
* `shared` depends only on `core`
* `infra` depends only on `shared + core`
* `app` wires infra → shared → engine
* `api` depends only on `engine + shared + AppState`

Violations of these rules are considered architectural bugs.

---

## Core (`core/`)

The core is **pure and deterministic**.

Contains:

* ECS entities and components
* Simulation time (`SimTime`, `SimDuration`)
* World definitions
* Physics, math, invariants

Rules:

* No HTTP
* No database structs
* No Supabase
* Serializable domain structs only

The core can be executed in isolation.

---

## Shared (`shared/`)

Defines **contracts and contexts**, never implementations.

Contains:

* `*Source` traits (ports)
* `RequestContext`, `AuthContext`
* Session and identity abstractions

Rules:

* No infra types
* No concrete storage
* No IO

---

## Infra (`infra/`)

Infra provides **implementations** of shared traits.

Current adapters:

* In-memory (dev / tests)
* JSON (world catalog + world state)

Future adapters:

* Supabase (auth, persistence)
* SQLite / Postgres

Rules:

* Implements `shared::*Source`
* Never called directly by API

---

## Engine (`engine/`)

Engines own **behavior and orchestration**.

Examples:

* `UserEngine`
* `WorldEngine`
* `TimeEngine`
* `LocationEngine`

Responsibilities:

* Coordinate sources
* Enforce invariants
* Contain business rules

Engines do NOT:

* Know about HTTP
* Know about databases
* Know about Supabase

---

## AppState

`AppState` is the **single source of truth** at runtime.

It contains:

* All sources (trait objects)
* All engines

Created **once** during bootstrap and shared via `Arc`.

Used by:

* API handlers (`State<AppState>`)
* Middleware (`Extension<AppState>`)

---

## API (`api/`)

The API is a **thin HTTP UI layer** built with Axum.

Responsibilities:

* Parse HTTP requests
* Call engines
* Return HTTP responses

Rules:

* No domain logic
* No infra access
* No Supabase usage

Example routes:

```
GET  /api/ping
GET  /api/time/*
POST /api/auth/signup
POST /api/auth/login
```

---

## Authentication & Identity Model

### Session-Based (Not Token-Based)

* Sessions are the root of trust
* Identity is derived from session state
* No JWTs in API
* No token refresh endpoints

### Flow

1. Anonymous session created
2. Signup or login associates user with session
3. Identity middleware resolves session → RequestContext
4. Handlers assume identity is already resolved

---

## Identity Middleware

Identity middleware runs on every request.

Responsibilities:

* Read `x-session-id` header
* Load session via `SessionSource`
* Inject `RequestContext`

Middleware does NOT:

* Verify tokens
* Talk to Supabase
* Perform auth logic

---

## World Data Model

Worlds are loaded from disk:

```
data/worlds/
├── earth.json
├── moon.json
└── sun.json
```

Each file contains an **array** of world definitions:

```json
[
  { "world_id": 1, "name": "Earth", ... }
]
```

This allows batching and future extensibility.

---

## Runtime Modes Supported

* Headless (no API)
* Offline (in-memory + JSON)
* HTTP server
* Future: Supabase-backed

---

## Architectural Guarantees

This architecture guarantees:

* Deterministic simulation
* Testability without mocks
* Infra swap without touching core
* Clear ownership boundaries
* Long-term maintainability

---

## Status

* API re-enabled ✔
* Auth decoupled ✔
* Infra optional ✔
* World loading stable ✔

---

This document should be kept up to date as the system evolves.
