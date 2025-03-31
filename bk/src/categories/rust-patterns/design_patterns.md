# Design Patterns

{{#include design_patterns.incl.md}}

## Implement an Abstract Factory {#abstract-factory}

See also:

- [abstract-factory in rust][abstract-factory-in-rust-website]⮳.

## Clone a Struct Storing a Boxed Trait Object {#dyn-clone}

[![dyn-clone][c-dyn_clone-badge]][c-dyn_clone]{{hi:dyn-clone}}
[![dyn-clone-crates.io][c-dyn_clone-crates.io-badge]][c-dyn_clone-crates.io]
[![dyn-clone-github][c-dyn_clone-github-badge]][c-dyn_clone-github]
[![dyn-clone-lib.rs][c-dyn_clone-lib.rs-badge]][c-dyn_clone-lib.rs]
[![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}
[![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}}

The [`dyn-clone`][c-dyn_clone]⮳{{hi:dyn-clone}} crate provides a `DynClone` trait that can be used in trait objects, and a [`clone_box`][c-clone_box]⮳{{hi:clone_box}} function that can clone any sized or dynamically sized implementation of `DynClone`. Types that implement the standard library's [`std::clone::Clone`][c-std::clone::Clone]⮳{{hi:std::clone::Clone}} trait are automatically usable by a `DynClone` trait object.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/design_patterns/dyn_clone.rs:example}}
```

## `pin-project` and `pin-project-lite` {#pin-project}

[![pin-project][c-pin_project-badge]][c-pin_project] [![pin-project-crates.io][c-pin_project-crates.io-badge]][c-pin_project-crates.io] [![pin-project-github][c-pin_project-github-badge]][c-pin_project-github] [![pin-project-lib.rs][c-pin_project-lib.rs-badge]][c-pin_project-lib.rs]{{hi:pin-project}}{{hi:Attribute}}{{hi:Macros}}{{hi:Pin}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[![pin-project-lite][c-pin_project_lite-badge]][c-pin_project_lite] [![pin-project-lite-crates.io][c-pin_project_lite-crates.io-badge]][c-pin_project_lite-crates.io] [![pin-project-lite-github][c-pin_project_lite-github-badge]][c-pin_project_lite-github] [![pin-project-lite-lib.rs][c-pin_project_lite-lib.rs-badge]][c-pin_project_lite-lib.rs]{{hi:pin-project-lite}}{{hi:Macros}}{{hi:Pin}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}}

[`pin-project`][c-pin_project]⮳{{hi:pin-project}} is a crate for safe and ergonomic pin-projection. The `#[pin_project]` attribute creates projection types covering all the fields of struct or enum.

`pin-project-lite` is a lightweight version of `pin-project` written with declarative macros. The `pin_project!` macro creates a projection type covering all the fields of struct.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/design_patterns/pin_project.rs:example}}
```

See also:

- [Projections and Structural Pinning](https://doc.rust-lang.org/std/pin/index.html#projections-and-structural-pinning)⮳.
- [std::pin::Pin](https://doc.rust-lang.org/std/pin/struct.Pin.html)⮳.

## Implement the Typestate Pattern in Rust {#typestate-pattern}

The typestate pattern is a design pattern that encodes information about an object's run-time state in its compile-time type.

- [Typestate Pattern][blog-typestate-pattern]⮳.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[design_patterns: rethink where that stuff should go](https://github.com/john-cd/rust_howto/issues/461)
</div>
