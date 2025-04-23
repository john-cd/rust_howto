# Attributes

{{#include attributes.incl.md}}

Attributes are annotations you attach to your Rust code, like functions, structs, modules, or even entire crates. They provide extra information or instructions to the Rust compiler or other tools (like linters or documentation generators). They don't change the logic of the code directly, but they influence how it's compiled, checked, or processed.

{{i:Attributes}} can take arguments with different syntaxes:

```rust,editable,compile_fail,noplayground
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes{{hi:Attributes}} `#![attr]` apply to the item that the attribute is declared within. You often see these at the very top of a file (`lib.rs` or `main.rs`) to apply to the entire crate or module.

## Lint Attributes {#lint-attributes}

During early development, place the following attributes at the top of `main.rs` or `lib.rs`:

```rust,editable
{{#include ../../crates/language/tests/feat/attributes_debug.rs:example}}
```

For production-ready code{{hi:Production-ready code}}, replace the above by the following, for example.

```rust,editable
{{#include ../../crates/language/tests/feat/attributes_production.rs:example}}
```

You also apply these attributes to specific [functions][p-functions]:

```rust,editable
{{#include ../../crates/language/tests/feat/allow_dead_code.rs:example}}
```

List of lint checks{{hi:Lint checks}}: `rustc -W help`. [`rustc`][rustc]{{hi:rustc}}⮳ also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`

## Automatically Derive Common Traits {#automatic-trait-derivation}

See [Automatic derivation][p-automatic-derivation].

## Mark as `must use` {#must-use}

```rust,editable
{{#include ../../crates/language/tests/feat/attributes_must_use.rs:example}}
```

## Mark as `deprecated` {#deprecated}

`deprecated`

```rust,editable
{{#include ../../crates/language/tests/feat/attributes_deprecated.rs:example}}
```

## Compile Conditionally {#conditional-compilation}

[Conditional compilation][book-rust-reference-conditional-compilation]⮳.

```rust,editable
{{#include ../../crates/language/tests/feat/conditional_compilation.rs:example}}
```

### `cfg-if` {#cfg-if}

[![cfg-if][c-cfg_if-badge]][c-cfg_if] [![cfg-if-crates.io][c-cfg_if-crates.io-badge]][c-cfg_if-crates.io] [![cfg-if-github][c-cfg_if-github-badge]][c-cfg_if-github] [![cfg-if-lib.rs][c-cfg_if-lib.rs-badge]][c-cfg_if-lib.rs]{{hi:cfg-if}}

A macro to ergonomically define an item depending on a large number of `#[cfg]` parameters. Structured like an if-else chain, the first matching branch is the item that gets emitted.

```rust,editable
{{#include ../../crates/language/tests/feat/cfg_if.rs:example}}
```

## Related Topics {#skip}

- [[derive | Derive]].
- [[rust-patterns | Rust Patterns]].
- [[testing | Testing]].

## References

- [Attributes reference][book-rust-reference-attributes]⮳.
- [![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[review NOW](https://github.com/john-cd/rust_howto/issues/985)
</div>
