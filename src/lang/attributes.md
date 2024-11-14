# Attributes

{{#include attributes.incl.md}}

## Attributes {#attributes}

{{i:Attributes}} can take arguments with different syntaxes:

```rust,compile_fail,noplayground
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes{{hi:Attributes}} `#![attr]` apply to the item that the attribute is declared within.

## Lint attributes {#lint-attributes}

During early development, place the following attributes at the top of `main.rs` or `lib.rs`

```rust
{{#include ../../deps/tests/lang/attributes_debug.rs:example}}
```

For production-ready code{{hi:Production-ready code}}, replace the above by the following, for example.

```rust
{{#include ../../deps/tests/lang/attributes_production.rs:example}}
```

You also apply these attributes to specific functions:

```rust
{{#include ../../deps/tests/lang/allow_dead_code.rs:example}}
```

List of lint checks{{hi:Lint checks}}: `rustc -W help`. [`rustc`][rustc]{{hi:rustc}}⮳ also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`

## Automatic trait derivation {#automatic-trait-derivation}

See [Automatic derivation][p-automatic-derivation].

## Must Use {#must-use}

```rust
{{#include ../../deps/tests/lang/attributes_must_use.rs:example}}
```

## Deprecated {#deprecated}

```rust
{{#include ../../deps/tests/lang/attributes_deprecated.rs:example}}
```

## Conditional Compilation {#conditional-compilation}

[Conditional compilation][book-rust-reference-conditional-compilation]⮳

```rust
{{#include ../../deps/tests/lang/conditional_compilation.rs:example}}
```

## See Also

[Attributes reference][book-rust-reference-attributes]⮳

[![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]

[p-automatic-derivation]: ../standard_library/derive.md
{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
