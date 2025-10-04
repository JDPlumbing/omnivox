# matcat Module

The `matcat` module provides a compact, extensible material catalog system for simulations and engineering applications. It enables efficient identification, procedural property generation, and similarity search for a wide range of materials using a small, deterministic code.

## Features

- **Compact Material IDs:**  
  Materials are identified by a 5-byte code (`MatCatId`) consisting of category, variant, and grade.
- **Procedural Property Generation:**  
  Material properties (`MatProps`) are deterministically generated from the ID, using category-based ranges and seeded pseudo-randomness.
- **Category and Variant Maps:**  
  Human-readable names for categories and variants are provided via static maps.
- **Similarity Search:**  
  Find the closest material to a target property set using Euclidean distance in property space.
- **Extensible:**  
  Easily add new categories, variants, or property ranges.

## Structure

- **mod.rs:** Module root and public API re-exports.
- **materials.rs:** Core types (`MatCatId`, `MatProps`) and property generation logic.
- **category_ranges.rs:** Defines property ranges for each material category and generates properties within those ranges.
- **categories.rs:** Maps category IDs to human-readable names.
- **variants.rs:** Maps (category, variant) pairs to variant names.

## Example Usage

```rust
use matcat::{MatCatId, props_for, find_closest_material};

// Create a material ID for Copper (category 1, variant 2, grade 1)
let copper_id = MatCatId::new(1, 2, 1);

// Get the procedural properties for Copper
let copper_props = props_for(&copper_id);

// Find the closest material to a target property set
let search_space = vec![copper_id /*, ... other MatCatIds ... */];
if let Some((best_id, best_props)) = find_closest_material(&copper_props, &search_space) {
    println!("Closest material: {:?} with props {:?}", best_id, best_props);
}