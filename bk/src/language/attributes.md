# Attributes

{{#include attributes.incl.md}}

[![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]

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

List of lint checks{{hi:Lint checks}}: `rustc -W help`. [`rustc`][rustc]{{hi:rustc}}⮳ also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`.

During early development, consider placing the following attributes at the top of `main.rs` or `lib.rs`:

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_debug.rs:example}}
```

For production-ready code{{hi:Production-ready code}}, replace the above by the following, for example.

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_production.rs:example}}
```

You also apply these attributes to specific [functions][p-functions]:

```rust,editable
{{#include ../../crates/language/tests/attributes/allow_dead_code.rs:example}}
```

## Automatically Derive Common Traits {#automatic-trait-derivation}

See [Automatic derivation][p-automatic-derivation].

## Mark as `must use` {#must-use}

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_must_use.rs:example}}
```

## Mark as `deprecated` {#deprecated}

When the Rust compiler sees code using an item marked as `#[deprecated]`, it will issue a warning during compilation.

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_deprecated.rs:example}}
```

## Compile Conditionally {#conditional-compilation}

[Conditional compilation][book-rust-reference-conditional-compilation]⮳.

```rust,editable
{{#include ../../crates/language/tests/attributes/conditional_compilation.rs:example}}
```

### Conditionally compile code blocks with `cfg-if` {#cfg-if}

[![cfg-if][c-cfg_if-badge]][c-cfg_if] [![cfg-if-crates.io][c-cfg_if-crates.io-badge]][c-cfg_if-crates.io] [![cfg-if-github][c-cfg_if-github-badge]][c-cfg_if-github] [![cfg-if-lib.rs][c-cfg_if-lib.rs-badge]][c-cfg_if-lib.rs]{{hi:cfg-if}}

`cfg-if` is a macro to ergonomically define an item depending on a large number of `#[cfg]` parameters. It is structured like an "if-else" chain. The first matching branch is the item that gets emitted.

```rust,editable
{{#include ../../crates/language/tests/attributes/cfg_if.rs:example}}
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
[finish review](https://github.com/john-cd/rust_howto/issues/985)
</div>
