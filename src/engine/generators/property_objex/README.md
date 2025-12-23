# property_objex

Location: `rust/omnivox/src/sim/generators/property_objex`

A small module that parses "objex" property descriptions and produces PropertyObject instances used by the simulation generators. Focus is on a compact, testable parser and a simple generator API that converts textual or file-backed objex representations into in-memory property objects.

## Responsibilities
- Parse the objex format (a lightweight, human-readable property exchange format).
- Validate required fields and basic constraints.
- Emit `PropertyObject` values suitable for downstream simulation code.
- Provide clear, testable error types for parsing/validation failures.

## Public API (summary)
- `Generator` — primary entry point for converting objex input into property objects.
- `Config` — generator options (e.g., default values, strict mode).
- `PropertyObject` — target struct produced by the generator.
- `Error` / `Result` — error types for parse/validation failures.
- Convenience helpers:
    - `generate_from_str(s: &str, cfg: &Config) -> Result<PropertyObject, Error>`
    - `generate_from_file(path: &Path, cfg: &Config) -> Result<PropertyObject, Error>`

Note: Check the module source for exact type names and signatures.

## Usage

Example (typical usage pattern):

```rust
use omnivox::sim::generators::property_objex::{Generator, Config};

let cfg = Config::default();
let mut gen = Generator::new(cfg);

// parse from a string
let input = r#"
name: "BoilerUnit"
type: "thermal"
properties:
    capacity: 1200
    efficiency: 0.88
"#;

let property = gen.generate_from_str(input)?;
```

Or from a file:

```rust
let property = gen.generate_from_file("tests/fixtures/boiler.objex")?;
```

Handle errors with pattern matching to present useful messages to callers.

## Testing
- Unit tests live alongside implementation.
- Run tests from project root:

```
cargo test --package omnivox
```

Add focused parsing tests for edge cases: missing required fields, malformed values, and strict vs permissive modes.

## Design notes / conventions
- Keep the parser deterministic and zero-allocation where practical.
- Prefer explicit validation errors (missing field, wrong type) over panic.
- The objex format is intentionally simple; extend only when there is clear demand.

## Contributing
- Follow existing crate style and run `cargo fmt` and `cargo clippy`.
- Add tests for any new behavior and update README if the public API changes.

## License
Follow the repository's license. No module-specific license is declared here.
