<!-- TODO Reviewed ??-??-2026. -->

# No Standard Library

{{#include no_std.incl.md}}

[`#![no_std]`][rust~no_std~attribute] prevents a crate from automatically linking Rust's standard library. The crate uses [`core`][c~core~docs], which provides primitive types, `Option`, `Result`, iterators, atomics, and formatting. `no_std` does not by itself prohibit heap allocation: a program can also use [`alloc`][c~alloc~docs] when its final binary supplies a global allocator.

## Write a library for both `std` and `no_std`

A library can enable `no_std` unless its `std` Cargo feature is selected. Dependencies that enable `std` by default should disable those defaults and re-enable the dependency's `std` feature from your crate's feature:

```toml
[features]
default = ["std"]
std = ["serde/std"]

[dependencies]
serde = { version = "1", default-features = false, features = ["derive"] }
```

Put the conditional attribute at the crate root (`src/lib.rs`), as in this workspace's `no_std` library:

```rust,ignore,noplayground
#![cfg_attr(not(feature = "std"), no_std)]

mod checksum;
pub use checksum::checksum;
```

The separate module implements a wrapping byte checksum without allocation or operating-system calls. This block belongs in the library module:

```rust,ignore,noplayground
{{#include ../../../crates/cats/no_std/src/checksum.rs:example}}
```

With these defaults, downstream users get `std` unless they disable default features. Omit `default = ["std"]` when downstream users should opt in to `std`. If the library needs a `Vec` or `String`, gate that API behind an `alloc` feature and use `alloc` types; the final application still chooses the allocator.

## Check that a crate builds without `std`

Check the dependency feature tree as well: one dependency that enables `std` by default can prevent the no-std build. The workspace's checksum library has no dependencies and a `std` feature enabled by default. From `bk/crates`, compile only its library without that feature, or run its separate host example:

```sh
cargo check --locked -p no_std --lib --no-default-features
cargo run --locked -p no_std --example no_std
```

A consumer can select the core-only library with `no_std = { path = "../no_std", default-features = false }` in its dependencies. The host example below uses `std` for printing; the library does not supply a panic handler or entry point, because those belong to the final binary.

## Run the checksum example on the host {#no-std}

```rust,ignore,noplayground
{{#include ../../../crates/cats/no_std/examples/no_std/checksum.rs:example}}
```

## Related Topics {#related-topics .skip}

- [No Allocation](no_alloc.md).

- [Dependencies](../../code-organization/dependencies.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
