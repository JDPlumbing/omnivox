# Omnivox Architecture

This document captures the **non‑obvious architectural rules** of the Omnivox system. These rules exist to preserve correctness, scalability, and sanity as the system grows.

If you are about to add a feature and something here feels inconvenient, **stop and re‑evaluate**. The inconvenience is usually protecting an invariant.

---

## 1. Core Mental Model

Omnivox is **not** a CRUD app.

It is a **world model**:

* entities exist independently of views
* meaning accrues over time
* simulation and observation are separate concerns

UI, API, and storage are *projections* over a persistent world state.

---

## 2. Entities

### Identity

* Every entity has a stable `EntityId` (UUID)
* IDs are opaque and never carry meaning
* IDs are returned from write operations and used for all future interaction

### Components

* Entities are identity + components (ECS)
* Components are **storage**, not intent
* Entities may exist without being simulated

### Important Rule

> Entities store **identity**, not **geometry**.

Spatial components (e.g. `Position`) store *addresses* (UvoxId), not physical vectors.

---

## 3. Spatial Model

### UvoxId

* Canonical spatial identity
* Stores scaled degrees + absolute radius
* Loss‑controlled, hashable, stable

### SurfaceCoords

* Human‑facing semantic location
* Uses radians internally
* Only used at boundaries

### Conversions

* `surface_to_uvox` and `uvox_to_surface` are the **only** places spatial conversion occurs
* Conversions require world + cosmic context

---

## 4. Units

### Internal Units

* Angles: **radians**
* Length: meters
* Time: nanoseconds (`SimTime`)

### External Units (API / UI)

* Angles: **degrees**
* Length: meters
* Time: seconds (f64)

### Rule

> Unit conversion happens **only at boundaries**.

Never convert units inside simulation or core logic.

---

## 5. Commands

Commands represent **intent**.

### Properties

* Commands mutate state
* Commands carry intent, not components
* Commands may create multiple entities
* Commands do not expose ECS internals

### Example

```rust
CreateMarker {
  world_id,
  location,
  note: String
}
```

Commands accept plain values and domain IDs — never ECS components.

---

## 6. Constructors

Constructors:

* create ECS components
* wire entities correctly
* contain no policy or permissions

Constructors are:

* pure
* reusable
* domain‑agnostic

---

## 7. Simulation Engine

The engine:

* owns time
* owns world, cosmic, environment, and entity state
* runs systems during `tick()`

### Important Rule

> Only the engine mutates simulation state.

---

## 8. API

### Write Endpoints

* call commands
* return `EntityId`
* do not return projections

### Read Endpoints

* project engine state
* are read‑only
* never mutate state

### Rule

> Writes use **commands**. Reads use **projections**.

---

## 9. UI

The UI:

* owns selection
* owns highlighting
* owns visibility
* owns grouping *as a view concern*

UI state is:

* ephemeral
* per‑user
* never stored in the engine

### Rule

> If the UI crashed and restarted, this state should not exist.

---

## 10. Environment

Environment:

* connects entities to world + cosmic state
* is resolved during simulation
* is not required for entity creation

Environment is a **simulation concern**, not a spatial one.

---

## 11. What Does NOT Belong in Core

* UI selection
* hover state
* auth / identity
* persistence adapters
* rendering

These live *above* the core.

---

## 12. Guiding Principle

> Facts live in the engine.
> Projections live outside it.

If unsure where something belongs, ask:

> "Does this affect physical reality or causality?"

If no — it does not belong in core.

---

## End

This document is a **guardrail**, not a straitjacket.

If a future feature requires breaking a rule, update this document first — intentionally.
