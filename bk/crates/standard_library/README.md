<!-- TODO Reviewed ??-??-2026 -->

# Standard Library

This crate contains comprehensive code examples demonstrating idiomatic usage of the Rust standard library (`std`).

## Examples Covered

- **Smart Pointers**: `Box`, `Rc`, `Arc`, `RefCell`, `Mutex`, and `RwLock`.
- **Collections & Iterators**: `Vec`, `HashMap`, `BTreeMap`, custom iterators, adapters (`map`, `filter`, `fold`), and collecting items.
- **Text & Strings**: String manipulation, `&str` slicing, UTF-8 handling, and formatting.
- **Error Handling & I/O**: Working with `Result`, `Option`, `std::io`, and `std::fs`.
- **Overflow** (`examples/overflow/`): Demonstrates checked, wrapping, overflowing, and saturating arithmetic.
- **Interior Mutability & Concurrency**: `Cell`, `RefCell`, `Atomic*` types, and standard thread primitives (`std::thread`).

## Usage

To check or test examples in this crate:

```console
cargo check --package standard_library --examples
cargo test --package standard_library
```

Code examples correspond to chapters in `bk/src/standard_library/`.
