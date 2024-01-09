# Attributes

Attributes can take arguments with different syntaxes:

```rust,editable,ignore
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes `#![attr]` apply to the item that the attribute is declared within.

## Lint attributes

During early development, place the following attributes at the top of `main.rs` or `lib.rs`

```rust,editable
{{#include ../../deps/examples/attributes_debug.rs}}
```

For production-ready code, replace the above by the following, for example.

```rust,editable
{{#include ../../deps/examples/attributes_production.rs}}
```

You also apply these attributes to specific functions:

```rust,editable
{{#include ../../deps/examples/allow_dead_code.rs}}
```

List of lint checks: `rustc -W help`. `rustc` also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`

## Automatic trait derivation

See [Automatic derivation](../concerns/derive.md)

## Must Use

```rust,editable
{{#include ../../deps/examples/attributes_must_use.rs}}
```

## Deprecated

```rust,editable
{{#include ../../deps/examples/attributes_deprecated.rs}}
```

## Conditional Compilation

[Conditional compilation][conditional-compilation]⮳

```rust,editable
{{#include ../../deps/examples/conditional_compilation.rs}}
```

## See Also

[Attributes reference][attributes-reference]⮳

[Rust by example - attributes][rust-by-example-attributes]⮳

[attributes-reference]: https://doc.rust-lang.org/reference/attributes.html
[conditional-compilation]: https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute
[rust-by-example-attributes]: https://doc.rust-lang.org/rust-by-example/attribute.html
{{#include ../links.md}}
