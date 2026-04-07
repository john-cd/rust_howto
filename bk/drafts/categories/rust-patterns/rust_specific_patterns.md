# Rust-specific Patterns

{{#include rust_specific_patterns.incl.md}}

## Clone a Struct Storing a Boxed Trait Object {#dyn-clone}

[![dyn-clone][c~dyn-clone~docs~badge]][c~dyn-clone~docs]{{hi:dyn-clone}}
[![dyn-clone~crates.io][c~dyn-clone~crates.io~badge]][c~dyn-clone~crates.io]
[![dyn-clone~repo][c~dyn-clone~repo~badge]][c~dyn-clone~repo]
[![dyn-clone~lib.rs][c~dyn-clone~lib.rs~badge]][c~dyn-clone~lib.rs]
[![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

The [`dyn-clone`][c~dyn-clone~docs]↗{{hi:dyn-clone}} crate provides a [`DynClone`][c~dyn-clone::DynClone~docs]↗{{hi:dyn-clone::DynClone}} trait that can be used in trait objects, and a [`clone_box`][c~clone_box~docs]↗{{hi:clone_box}} function that can clone any sized or dynamically sized implementation of `DynClone`. Types that implement the standard library's [`std::clone::Clone`][c~std::clone::Clone~docs]↗{{hi:std::clone::Clone}} trait are automatically usable by a `DynClone` trait object.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/dyn_clone.rs:example}}
```

## `pin-project` and `pin-project-lite` {#pin-project}

[![pin-project][c~pin-project~docs~badge]][c~pin-project~docs] [![pin-project~crates.io][c~pin-project~crates.io~badge]][c~pin-project~crates.io] [![pin-project~repo][c~pin-project~repo~badge]][c~pin-project~repo] [![pin-project~lib.rs][c~pin-project~lib.rs~badge]][c~pin-project~lib.rs]{{hi:pin-project}}{{hi:Attribute}}{{hi:Macros}}{{hi:Pin}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[![pin-project-lite][c~pin-project-lite~docs~badge]][c~pin-project-lite~docs] [![pin-project-lite~crates.io][c~pin-project-lite~crates.io~badge]][c~pin-project-lite~crates.io] [![pin-project-lite~repo][c~pin-project-lite~repo~badge]][c~pin-project-lite~repo] [![pin-project-lite~lib.rs][c~pin-project-lite~lib.rs~badge]][c~pin-project-lite~lib.rs]{{hi:pin-project-lite}}{{hi:Macros}}{{hi:Pin}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}}

[`pin-project`][c~pin-project~docs]↗{{hi:pin-project}} is a crate for safe and ergonomic pin-projection. The `#[pin_project]` attribute creates projection types covering all the fields of struct or enum.

`pin-project-lite` is a lightweight version of `pin-project` written with declarative macros. The `pin_project!` macro creates a projection type covering all the fields of struct.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/rust_specific_patterns/pin-project.rs:example}}
```

See also:

- [Projections and Structural Pinning][c~std::pin~projections-and-structural-pinning~docs]↗.
- [`std::pin::Pin`][c~std::pin::Pin~docs]↗.

## Related Topics {#related-topics .skip}

FIXME

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1393)

- [& vs. ref in Rust patterns][blog~rust-patterns-ref]↗.

</div>
