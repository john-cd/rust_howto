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

```rust,editable,ignore
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
// or simply #[allow(unused)]
#![allow(dead_code)]
#[allow(missing_docs)]
```

For production-ready code, replace the above by the following, for example.

```rust,editable,ignore
#![warn(
    unused,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    rust_2018_idioms,
)]
#![deny(unreachable_pub)]  // error if violation
#![forbid(unsafe_code)]    // same as `deny` +forbids changing the lint level afterwards
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
{{#include ../../deps/examples/must_use.rs}}
```

## Deprecated

```rust,editable,ignore
#[deprecated(since = "5.2.0", note = "use bar instead")]
pub fn foo() {}
```

## Conditional Compilation

[Conditional compilation]( https://doc.rust-lang.org/reference/conditional-compilation.html#the-cfg-attribute )

```rust,editable
{{#include ../../deps/examples/conditional_compilation.rs}}
```

## See Also

[Attributes reference]( https://doc.rust-lang.org/reference/attributes.html )

[Rust by example - attributes]( https://doc.rust-lang.org/rust-by-example/attribute.html )
