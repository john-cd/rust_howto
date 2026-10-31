<!-- TODO Reviewed ??-??-2026 -->

# Builder Pattern

{{#include builder_pattern.incl.md}}

## Construct a Complex Struct with a Builder {#basic-builder-pattern}

Use a builder when construction has many optional settings, needs validation, or benefits from readable call-site configuration. Keep required values required in the constructor or use a typestate builder; represent optional values with `Option<T>` or defaults. A handwritten builder gives direct control over validation and error types, while derive crates reduce repetitive code. Prefer a simple constructor when there are only a few required fields.

- Instead of creating the object directly, you create a separate builder, typically named `XyzBuilder` after the type being built. The builder holds the state needed to construct the final object, often storing optional fields as `Option<T>`.
- The builder provides methods (often named after the fields they set, like `.name("Example")`, `.timeout(Duration::from_secs(5))`) to configure the object piece by piece. These methods usually return `self`, allowing chaining calls together fluently: `builder.field_a(value_a).field_b(value_b)`.
- Once the desired fields are set, call a final method on the builder (commonly named `.build()`). It performs any necessary validation and returns the completed object, or a `Result` when validation can fail.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/examples/builder_pattern/builder_pattern.rs:example}}
```

## `typed-builder` {#typed-builder}

{{#badges typed-builder rust-patterns="Rust patterns"}}{{hi:Builder}}

[`typed-builder`][c~typed-builder~docs]↗{{hi:typed-builder}} derives compile-time type-checked builders. It uses a 'derive' macro and leverages Rust's type system to ensure that all required fields are set. If you forget a required field, your code won't even compile.

## `derive_builder` {#derive_builder}

{{#badges derive_builder development-tools="Development tools" rust-patterns="Rust patterns"}}{{hi:Builder}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}}{{hi:Struct}}

[`derive_builder`][c~derive_builder~docs]↗{{hi:derive_builder}} provides a `#[derive(Builder)]` macro to automatically implement the builder pattern for arbitrary structs. It performs checks at runtime within the `.build()` method.

## `bon` {#bon}

[![bon~website][c~bon~website~badge]][c~bon~website] [![bon][c~bon~docs~badge]][c~bon~docs] [![bon~crates.io][c~bon~crates.io~badge]][c~bon~crates.io] [![bon~repo][c~bon~repo~badge]][c~bon~repo] [![bon~lib.rs][c~bon~lib.rs~badge]][c~bon~lib.rs]{{hi:bon}}{{hi:Builder}}{{hi:Constructor}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}} [![cat~no-std::no-alloc][cat~no-std::no-alloc~badge]][cat~no-std::no-alloc]{{hi:No dynamic allocation}} [![cat~rust-patterns][cat~rust-patterns~badge]][cat~rust-patterns]{{hi:Rust patterns}}

[`bon`][c~bon~docs]↗{{hi:bon}} is a compile-time-checked builder generator with additional features like support for fallible/async builders and named function arguments via the builder pattern.

## Related Topics {#related-topics .skip}

- [Rust Patterns](index.md) groups builders with other creational, structural, and behavioral patterns.
- [Error Handling](error_handling/error_handling.md) covers reporting validation failures from `build`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
- [Next-gen builder macro Bon Revolutional typestate design][blog~bon-v3-release]↗.

</div>
