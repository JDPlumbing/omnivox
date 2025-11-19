# droidid Module

The `droidid` module provides a simple utility for generating random, droid-style IDs reminiscent of classic sci-fi robots (e.g., `R2-D2`, `X9C3`). These IDs are useful for assigning unique, human-friendly identifiers to agents, bots, or simulated entities in your application.

## Features

- **Randomized Format:** Generates IDs with a mix of uppercase letters, lowercase letters, and digits.
- **Optional Dash:** May insert a dash at a random position (not at the start or end) for a more "droid-like" appearance.
- **Configurable Length:** ID length varies randomly between 4 and 6 characters.

## Example

```rust
use droidid::generate;

let id = generate();
println!("Generated droid ID: {}", id); // e.g., "X9C3" or "R2-D2"