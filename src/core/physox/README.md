# physox

physox is a small Rust module within the Omnivox project that provides low-level physical-layer utilities and abstractions for embedded and system-level code. It centralizes device I/O primitives, mocks for testing, and helpers for timing and platform-specific bindings so higher-level modules can remain platform-agnostic.

## Key goals
- Provide a thin, well-documented abstraction over serial, GPIO, and timing primitives.
- Offer test-friendly mocks and simulators for unit tests.
- Keep the public API minimal and ergonomic for consumers inside the Omnivox codebase.

## Features
- Serial/UART wrapper with configurable baud, parity, and timeouts.
- GPIO traits and simple concrete implementations.
- Timer utilities and delay helpers.
- Test doubles / mock implementations for CI and unit tests.
- Small, dependency-light design suitable for embedding into other crates.

## Installing / using
This module is intended to be used as an internal crate within the Omnivox workspace. Add it as a path dependency in your workspace Cargo.toml, for example:

```toml
[dependencies]
physox = { path = "rust/omnivox/src/physox" }
```

Then import the module in your crate:

```rust
use physox::{SerialPort, GpioPin, Delay};
```

## Quick example

```rust
// Example shows a hypothetical API surface. Adapt to actual functions/types.
fn send_heartbeat<S: SerialPort>(port: &mut S) -> Result<(), S::Error> {
    port.write_all(b"heartbeat\n")?;
    port.flush()?;
    Ok(())
}

fn toggle_led<P: GpioPin>(pin: &mut P) {
    pin.set_high();
    Delay::ms(100);
    pin.set_low();
}
```

## Testing
- Use provided mock implementations in tests to avoid depending on hardware.
- Run unit tests with `cargo test` from the workspace root.
- For integration tests that require hardware, add conditional features or environment-controlled test flags.

## Contributing
- Keep changes focused and well-tested.
- Add unit tests for new behaviors and mocks.
- Follow the project coding conventions and run `cargo fmt` and `cargo clippy` before submitting PRs.

## Documentation
- Public API is documented inline with Rustdoc. Generate with:
  ```
  cargo doc --open
  ```

## License
- Matches the Omnivox project license. See the workspace LICENSE file for details.

If you need a tailored README with exact API examples from the current module code, provide the public function/type list or paste the module source and a more precise README will be generated.