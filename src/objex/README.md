# objex Module

The `objex` module provides the core logic, data structures, and property systems for simulation objects ("objex") within the Omnivox platform. It enables the creation, persistence, and physical property derivation of objects and composites, supporting extensible simulation of real-world materials and behaviors.

## Features

- **Object Modeling:**  
  - `Object<T>`: Generic struct for simulation objects, parameterized by shape.
  - `CompositeObject<T>`: Objects composed of multiple layers/materials.
  - `Objex`: Serializable, database-friendly object representation.
- **Physical Property Systems:**  
  - Mass, strength, mechanical, thermal, electrical, and degradation property derivation.
  - Composite property derivation for layered objects.
- **Persistence:**  
  - Async functions for inserting and fetching objects from a Supabase backend.
- **Error Handling:**  
  - Unified `ObjexError` type for all operations.
- **Extensible:**  
  - Easily add new property systems or extend object types.

## Structure

- **core/**: Core object and composite definitions.
  - `object.rs`: Defines `Object<T>`.
  - `composite.rs`: Defines `CompositeObject<T>`.
  - `types.rs`: Defines `Objex`, `Shape`, `MaterialLink`, etc.
- **systems/**: Physical property derivation modules.
  - `mass.rs`, `strength.rs`, `mechanical.rs`, `thermal.rs`, `electrical.rs`, `degradation.rs`, `composite.rs`
- **persist.rs**: Async persistence helpers for Supabase.
- **error.rs**: Error types and result alias.
- **builder.rs**: Fluent builder for constructing objects.
- **defaults.rs**: Default values and helpers.

## Example Usage

```rust
use objex::{Object, ObjexBuilder, persist::insert_objex, systems::mass::derive_mass};
use geospec::shapes::BoxShape;
use matcat::MatCatId;

// Build an object
let shape = BoxShape { width: 1.0, height: 2.0, depth: 3.0 };
let material = MatCatId::new(1, 2, 1); // Example material
let obj = ObjexBuilder::new("My Box", shape, material).build();

// Derive mass properties
let mass_props = derive_mass(&obj);

// Persist to Supabase
// insert_objex(&supa, &obj).await?;