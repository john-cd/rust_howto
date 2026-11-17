<!-- TODO Reviewed ??-??-2026. -->

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

## Compile-time Checked Builders {#typed-builder}

{{#badges typed-builder rust-patterns="Rust patterns"}}{{hi:Builder}}

[`typed-builder`][c~typed-builder~docs]↗{{hi:typed-builder}} derives a builder whose type tracks which required fields have been set. A missing required field is a compile error when the builder is finalized.

## `derive_builder` {#derive_builder}

{{#badges derive_builder development-tools="Development tools" rust-patterns="Rust patterns"}}{{hi:Builder}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}}{{hi:Struct}}

[`derive_builder`][c~derive_builder~docs]↗{{hi:derive_builder}} derives a builder for a struct. Its `.build()` method checks at runtime that required fields are present.

## `bon` {#bon}

[![bon~website][c~bon~website~badge]][c~bon~website] {{#badges bon}} {{hi:Builder}} {{hi:Constructor}} {{hi:Derive}} {{hi:Macro}} {{hi:Setter}} {{#categories asynchronous=Asynchronous,data-structures="Data structures",no-std="No standard library",no-std::no-alloc="No dynamic allocation",rust-patterns="Rust patterns"}}

[`bon`][c~bon~docs]↗{{hi:bon}} derives builders with support for required fields, fallible or asynchronous construction, and named arguments for functions.

## Related Topics {#related-topics .skip}

- [Rust Patterns](index.md) groups builders with other creational, structural, and behavioral patterns.
- [Recoverable Error Handling](../../standard-library/recoverable_error_handling.md) covers returning validation failures from `build`.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
- [Next-gen builder macro Bon Revolutional typestate design][blog~bon-v3-release]↗.

</div>
