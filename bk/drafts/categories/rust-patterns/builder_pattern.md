# Builder Pattern

{{#include builder_pattern.incl.md}}

## Construct a Complex Struct with a Builder {#basic-builder-pattern}

The builder pattern provides a clean and readable way to construct complex objects, especially when dealing with multiple optional fields or validation rules.

- Instead of creating the object directly, you first create a separate Builder object. It is typically named `xyzBuilder` where `xyz` is the name of the struct you're building. The builder holds the state needed to construct the final object, often storing fields as `Option<T>` internally.
- The builder provides methods (often named after the fields they set, like `.name("Example")`, `.timeout(Duration::from_secs(5))`) to configure the object piece by piece. These methods usually return `self`, allowing you to chain calls together fluently: `builder.field_a(value_a).field_b(value_b)`.
- Once the desired fields are set, you call a final method on the builder (commonly named `.build()` or `.finish()`). This method takes the configuration stored in the builder, performs any necessary validation (like checking if required fields were provided), and then constructs and returns the final, fully-formed object. If the validation can fail, the `build` method typically returns a `Result`.

```rust,editable
{{#include ../../../crates/cats/rust_patterns/tests/builder_pattern/builder_pattern.rs:example}}
```

## `typed-builder` {#typed-builder}

[![typed-builder][c-typed_builder-badge]][c-typed_builder] [![typed-builder-crates.io][c-typed_builder-crates.io-badge]][c-typed_builder-crates.io] [![typed-builder-github][c-typed_builder-github-badge]][c-typed_builder-github] [![typed-builder-lib.rs][c-typed_builder-lib.rs-badge]][c-typed_builder-lib.rs]{{hi:typed-builder}}{{hi:Builder}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`typed-builder`][c-typed_builder]⮳{{hi:typed-builder}} lets you derive compile-time type-checked builders. It uses a derive macro and leverages Rust's type system to ensure that all required fields are set. If you forget a required field, your code won't even compile.

## `derive_builder` {#derive_builder}

[![derive_builder][c-derive_builder-badge]][c-derive_builder] [![derive_builder-crates.io][c-derive_builder-crates.io-badge]][c-derive_builder-crates.io] [![derive_builder-github][c-derive_builder-github-badge]][c-derive_builder-github] [![derive_builder-lib.rs][c-derive_builder-lib.rs-badge]][c-derive_builder-lib.rs]{{hi:derive_builder}}{{hi:Builder}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}}{{hi:Struct}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`derive_builder`][c-derive_builder]⮳{{hi:derive_builder}} provides a `#[derive(Builder)]` macro to automatically implement the builder pattern for arbitrary structs. It performs checks at runtime within the `.build()` method.

## `bon` {#bon}

[![bon-website][c-bon-website-badge]][c-bon-website] [![bon][c-bon-badge]][c-bon] [![bon-crates.io][c-bon-crates.io-badge]][c-bon-crates.io] [![bon-github][c-bon-github-badge]][c-bon-github] [![bon-lib.rs][c-bon-lib.rs-badge]][c-bon-lib.rs]{{hi:bon}}{{hi:Builder}}{{hi:Constructor}}{{hi:Derive}}{{hi:Macro}}{{hi:Setter}} [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-data-structures][cat-data-structures-badge]][cat-data-structures]{{hi:Data structures}} [![cat-no-std][cat-no-std-badge]][cat-no-std]{{hi:No standard library}} [![cat-no-std::no-alloc][cat-no-std::no-alloc-badge]][cat-no-std::no-alloc]{{hi:No dynamic allocation}} [![cat-rust-patterns][cat-rust-patterns-badge]][cat-rust-patterns]{{hi:Rust patterns}}

[`bon`][c-bon]⮳{{hi:bon}} is a compile-time-checked builder generator with additional features like support for fallible/async builders and named function arguments via the builder pattern.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write and add to index and SUMMARY](https://github.com/john-cd/rust_howto/issues/648)

- [Next-gen builder macro Bon Revolutional typestate design](https://bon-rs.com/blog/bon-v3-release)

</div>
