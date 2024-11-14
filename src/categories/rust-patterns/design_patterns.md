# Design Patterns

{{#include design_patterns.incl.md}}

## Abstract factory {#abstract-factory}

[abstract-factory in rust][abstract-factory-in-rust-website]⮳

## `dyn-clone` {#dyn-clone}

[![dyn-clone][c-dyn_clone-badge]][c-dyn_clone]{{hi:dyn-clone}}
[![dyn-clone-crates.io][c-dyn_clone-crates.io-badge]][c-dyn_clone-crates.io]
[![dyn-clone-github][c-dyn_clone-github-badge]][c-dyn_clone-github]
[![dyn-clone-lib.rs][c-dyn_clone-lib.rs-badge]][c-dyn_clone-lib.rs]
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

This crate provides a `DynClone` trait that can be used in trait objects, and a `clone_box` function that can clone any sized or dynamically sized implementation of `DynClone`. Types that implement the standard library’s `std::clone::Clone` trait are automatically usable by a DynClone trait object.

```rust
{{#include ../../../deps/tests/cats/rust_patterns/dyn_clone.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO rethink where that stuff should go

TODO review - lens is not used that often:

## Lens {#lens}

[![lens_rs][c-lens_rs-badge]][c-lens_rs]{{hi:lens_rs}}
[![lens_rs-crates.io][c-lens_rs-crates.io-badge]][c-lens_rs-crates.io]
[![lens_rs-github][c-lens_rs-github-badge]][c-lens_rs-github]
[![lens_rs-lib.rs][c-lens_rs-lib.rs-badge]][c-lens_rs-lib.rs]
</div>
