# droidid

[![CI](https://github.com/JDPlumbing/droidid/actions/workflows/ci.yml/badge.svg)](https://github.com/JDPlumbing/droidid/actions)

Created by [JDPlumbing](https://github.com/JDPlumbing)
Generate short, quirky droid-style IDs like `R2-D2`, `M8iwB`, or `X7qL`.  
Inspired by classic sci-fi naming conventions, `droidid` gives you millions of unique, human-readable identifiers.

---

## ✨ Features
- Generates IDs 4–6 characters long.
- Uses uppercase, lowercase, and digits.
- Optionally inserts a dash (never first or last).
- Huge possibility space — millions of unique IDs.
- Perfect for unique short handles, test data, or just for fun.

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
droidid = "0.1"
```

Or install locally for development:

```bash
cargo add droidid
```

---

## 🚀 Usage

In code:

```rust
use droidid::generate;

fn main() {
    let id = generate();
    println!("{}", id); // e.g. "R2-D2"
}
```

Run the included example CLI:

```bash
cargo run --example cli
# Output:
# R2-D2
# M8iwB
# q7-Lp
```

---

## 📊 Example outputs

Some generated IDs:
```
R2-D2
M8iwB
X7qL
b9-Gh
T3oP
```

---

## ⚖️ License

MIT License. See [LICENSE](LICENSE) for details.
