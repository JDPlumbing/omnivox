# Architecture & Layering Rules

This project follows a strict, DB-agnostic, ECS-style architecture.
These rules exist to prevent persistence, API, or framework concerns
from leaking into core logic.

If something feels annoying to wire up, that is usually a sign the
boundary is doing its job.

---

## Layer Overview

### `core/`
**Domain & truth**

- Defines *what exists* and *what is true*
- ECS entities, components, domain structs
- Engines (state mutation with invariants)
- Systems (read-only derivations)

**Rules**
- No database knowledge
- No API knowledge
- No IO
- No async
- No Supabase / JSON / SQL types
- Domain structs live here

**Examples:**
```rust
Property
Address
WorldSummary
EntityId
Position
```

---

### `engine/`
**Behavior & rules**

- Mutates core state via engines
- Enforces invariants
- May fail

**Rules**
- Operates only on core data
- No persistence logic
- No transport logic

---

### `shared/`
**Interfaces & wiring**

Only two kinds of things are allowed here:

#### `*Source`
Traits that provide access to data
- Return core domain structs
- Abstract where data comes from

**Examples:**
- `WorldCatalog`
- `WorldStateSource`
- `PropertySource`

#### `*Context`
Bundles capabilities needed to run logic
- Passed into engines or runners
- No domain data stored

**Examples:**
- `WorldContext`
- `SimulationContext`

**Rules**
- No domain structs
- No DB structs
- No DTOs
- Traits and thin wiring only

---

### `infra/`
**Persistence & transport**

- Database access
- JSON files
- Supabase / SQL / external services

#### Database structs
Any struct that exists for persistence must end with `Row`.

**Examples:**
- `PropertyRow`
- `AddressRow`
- `WorldRow`

**Rules**
- DB row structs live here
- Conversion to/from core happens here
- Infra may depend on core
- Core never depends on infra

---

### `api/`
**Input / Output layer**

- HTTP handlers
- Request / response types
- DTOs and query structs

**Examples:**
- `PropertyQuery`
- `PropertyResponse`
- `PropertyDto`

**Rules**
- No domain logic
- No persistence logic
- Maps to/from core types

---

## Naming Rules (Strict)

- `*Row` → persistence only (`infra/`)
- Domain structs → `core/`
- `*Source`, `*Context` → `shared/`
- DTOs / Queries / Responses → `api/`

If a name doesn't fit these patterns, stop and re-evaluate.

---

## Dependency Direction (Never Violated)

```
infra  → core
api    → shared → core
engine → core
core   → nothing
```

If something in `core/` needs a DB, HTTP, or file system — the design is wrong.

---

## Guiding Principle

Core logic must be runnable in memory with:

- No database
- No API
- No filesystem
- No async

**Persistence, APIs, and UIs are projections, not foundations.**
