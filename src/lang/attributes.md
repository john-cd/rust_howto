# Attributes

Attributes can take arguments with different syntaxes:

```rust,ignore
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes{{hi:Attributes}} `#![attr]` apply to the item that the attribute is declared within.

## Lint attributes

During early development, place the following attributes at the top of `main.rs` or `lib.rs`

```rust
{{#include ../../deps/tests/attributes_debug.rs}}
```

For production-ready code{{hi:Production-ready code}}, replace the above by the following, for example.

```rust
{{#include ../../deps/tests/attributes_production.rs}}
```

You also apply these attributes to specific functions:

```rust
{{#include ../../deps/tests/allow_dead_code.rs}}
```

List of lint checks{{hi:Lint checks}}: `rustc -W help`. [`rustc`][rustc]{{hi:rustc}}⮳ also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`

## Automatic trait derivation

See [Automatic derivation][p-automatic-derivation].

## Must Use

```rust
{{#include ../../deps/tests/attributes_must_use.rs}}
```

## Deprecated

```rust
{{#include ../../deps/tests/attributes_deprecated.rs}}
```

## Conditional Compilation

[Conditional compilation][book-rust-reference-conditional-compilation]⮳

```rust
{{#include ../../deps/tests/conditional_compilation.rs}}
```

## See Also

[Attributes reference][book-rust-reference-attributes]⮳

[![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]

[p-automatic-derivation]: ../standard_library/derive.md
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
