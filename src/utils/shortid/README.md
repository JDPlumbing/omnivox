# shortid-rs
[![Crates.io](https://img.shields.io/crates/v/shortid-rs.svg)](https://crates.io/crates/shortid-rs)
[![Docs.rs](https://docs.rs/shortid-rs/badge.svg)](https://docs.rs/shortid-rs)
[![Build Status](https://github.com/yourname/shortid-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/yourname/shortid-rs/actions)

Generate short, human-friendly IDs derived from UUIDs with collision handling.  
Great for games, simulations, or any system where raw UUIDs are too long and ugly to show users.

---

## ✨ Features
- Deterministically derive a short ID from a UUID.
- Compact 6-char Base62 encoding (A–Z, a–z, 0–9).
- Collision handling with automatic `-` or `_` insertion (never first or last).
- Fast: ~1µs per ID generation in benchmarks.
- Keeps full UUIDs as primary keys internally; short IDs are for human use.

---

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
shortid-rs = "0.1.0"
uuid = { version = "1", features = ["v4"] }
```

---

## 🚀 Usage

```rust
use shortid_rs::{short_code_from_uuid, unique_short_code};
use uuid::Uuid;
use std::collections::HashSet;

fn main() {
    let uuid = Uuid::new_v4();
    let mut existing = HashSet::new();

    let code = unique_short_code(&uuid, &mut existing);
    println!("UUID: {} -> Short ID: {}", uuid, code);

    let deterministic = short_code_from_uuid(&uuid);
    println!("Deterministic short code: {}", deterministic);
}
```

Example output:

```
UUID: 550e8400-e29b-41d4-a716-446655440000 -> Short ID: Ab9xQ2
Deterministic short code: Ab9xQ2
```

---

## 🧪 Tests

Run the built-in tests:

```bash
cargo test
```

Or run integration tests in the `tests/` folder:

```bash
cargo test --test basic
```

---

## 📊 Benchmarks

Benchmarks are provided with [Criterion.rs](https://bheisler.github.io/criterion.rs/book/).

```bash
cargo bench
```

Example result:

```
generate_short_code     time:   [911.40 ns 1.38 µs 2.40 µs]
```

---

## ⚖️ License

MIT License. See [LICENSE](LICENSE) for details.
