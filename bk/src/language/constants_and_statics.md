<!-- TODO Reviewed ??-??-2026 -->

# Constants and Statics

{{#include constants_and_statics.incl.md}}

[![Rust by Example - constants][book~rust-by-example~constants~badge]][book~rust-by-example~constants]

Both `const` (constants) and `static` (static variables) are used to declare values that live for the entire duration of a program.

## Declare Constants {#constants}

Constants, declared using the `const` keyword, are values that are inlined directly into the code wherever they are used. Think of them as symbolic names for literal values that are known at compile time.

- By convention, `const` names are in SCREAMING_SNAKE_CASE.
- Constants are always immutable. You _cannot_ declare a `const` `mut`.
- Unlike `let` bindings (where type inference is common), you must explicitly specify the type of a `const`.
- Constants can be declared in any scope: While often found at the top level, constants can be declared inside functions, modules, or other blocks.
- The value of a `const` must be a constant expression, meaning it can be fully evaluated at compile time. This includes literals, arithmetic operations on literals, and calls to constant functions (which are declared as `const fn` instead of `fn` and are limited in what they can do).
- The only lifetime allowed in a constant is `'static`, which is the lifetime that encompasses all others in a Rust program.
- `const` values do not have a dedicated memory address. When you use a constant, the compiler essentially "copies and pastes" its value directly into the machine code at each usage site.

Constants are primarily used for values that truly never change throughout the lifetime of your program, such as mathematical constants or configuration values.

```rust,editable
{{#include ../../crates/language/examples/constants_and_statics/constants.rs:example}}
```

### References {#references .skip}

- [Constants (Rust book)][book~rust~ch03-01-variables-and-mutability-constants]↗.
- [Constant evaluation (Rust reference)][book~rust-reference~const_eval]↗.

## Declare Statics {#statics}

Static variables (declared with `static`) represent a single, fixed location in memory for the entire duration of the program. They are effectively global variables.

- By convention, static names are in SCREAMING_SNAKE_CASE.
- You must explicitly specify the type of the `static` in its declaration.
- `static` variables must be initialized with a constant expression (made of literals, constants, calls to `const fn` functions...).
- Static items have a consistent, fixed memory address throughout the program's execution.
- They have the `'static` lifetime, meaning they live for the entire duration of the program.
- Unless it's a `static mut` (see below), the type of a `static` variable generally needs to implement the `Sync` trait, ensuring thread-safe access. This prevents unsynchronized access to data that could lead to corruption.
- `Drop` behavior: static variables are not dropped when the program exits; the operating system reclaims the process memory. For thread-safe lazy initialization, use [`std::sync::LazyLock`][c~std::sync::LazyLock~docs]↗ (stable since Rust 1.80), but a lazy static is still not dropped at process exit. If a resource needs explicit cleanup, give it an owner with a defined lifetime and close or drop it explicitly.

```rust,editable
{{#include ../../crates/language/examples/constants_and_statics/statics.rs:example}}
```

Statics can be mutable (`static mut`): Unlike `const`, static variables can be declared as mutable using `static mut`. However, accessing or modifying a `static mut` variable is `unsafe`, because it can lead to data races in a multi-threaded environment. Consider `static mut` only as a last resort.

In the Rust 2024 edition, the `static_mut_refs` lint is denied by default: code that creates a shared or mutable reference to a `static mut` is rejected. Prefer an immutable `static` with an atomic, lock, `OnceLock`, or `LazyLock` when global state is needed ([Rust 2024 Edition Guide][book~edition-guide~rust-2024~static-mut-references]↗).

```rust,editable
{{#include ../../crates/language/examples/constants_and_statics/static_mut.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[data_types | Data Types]].
- [[lifetimes | Lifetimes]].
- [[variables | Variables]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">

[When are Rust's `const fn`s executed?][blog~const-fn]↗.

accuracy review: introduction and language basics

</div>
