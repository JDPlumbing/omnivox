# Omnivox: Modular Simulation Platform

Omnivox is a modular, extensible simulation platform written in Rust. It provides a set of interoperable crates and modules for simulating worlds, objects, timelines, materials, spatial math, and more. Each module is designed to be reusable and composable, enabling rich scientific, engineering, or game simulations.

## Modules Overview

Below is a summary of each major module in the Omnivox workspace and its purpose:

---

### **chronovox**
**Event sourcing, timeline, and state playback engine.**  
Models all changes in the simulation as time-stamped events, enabling robust replay, querying, and state reconstruction for any entity. Includes event types, timeline playback, persistence, and error handling.

---

### **droidid**
**Random droid-style ID generator.**  
Utility for generating unique, human-friendly IDs reminiscent of sci-fi robots (e.g., `R2-D2`, `X9C3`). Useful for naming agents, bots, or simulated entities.

---

### **geospec**
**Geometric primitives and inference utilities.**  
Provides shapes (sphere, box, cylinder, etc.), traits for surface area and volume, and functions for inferring missing properties from partial JSON descriptions.

---

### **matcat**
**Compact material catalog system.**  
Defines a 5-byte material ID, procedural property generation, and similarity search for materials. Supports extensible categories, variants, and property ranges.

---

### **objex**
**Simulation object modeling and property systems.**  
Defines core object types, composite objects, and systems for deriving physical properties (mass, strength, mechanical, thermal, electrical, degradation). Supports persistence and extensibility.

---

### **sim**
**Core simulation engine and world management.**  
Manages simulation state, world data, event timelines, and modular systems (movement, physics, etc.). Handles loading, running, and persisting simulation sessions.

---

### **supabasic**
**Async Supabase client and data access layer.**  
Provides a lightweight client for interacting with Supabase, with ORM-like helpers for CRUD operations on users, worlds, simulations, events, and objects.

---

### **tdt**
**Time Delta Toolkit.**  
Utilities for representing, manipulating, and formatting time intervals. Supports human-friendly formatting and tick counting in various units.

---

### **uvoxid**
**Compact spatial identifier for reference frames.**  
Represents locations as 4x64-bit fields (frame, radius, latitude, longitude) with arithmetic and serialization support. Enables precise, lossless spatial math.

---

### **uvoxxyz**
**3D spatial math and coordinate conversion.**  
Converts between `UvoxId` and Cartesian coordinates, supports multiple coordinate systems, ENU (East-North-Up) frames, and quaternion-based orientation.

---

## Getting Started

Each module contains its own README with usage examples and API documentation.  
To use a module, add it as a dependency in your Cargo.toml and follow the examples provided.

---

**Maintainer:** drippy  
**License:** MIT