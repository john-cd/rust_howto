# Attributes

{{#include attributes.incl.md}}

## Attribute Syntax {#skip}

[![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]{{hi:Attributes}}

Attributes are annotations you attach to your Rust code, like functions, structs, modules, or even entire crates. They provide extra information or instructions to the Rust compiler or other tools (like linters or documentation generators). They don't change the logic of the code directly, but they influence how it's compiled, checked, or processed.

Common attributes include:

- `#[derive(...)]` to autogenerate an implementation of certain traits.
- `#[test]` to mark functions as tests.
- `#[allow(...)]`, `#[warn(...)]`, `#[deny(...)]` to control compiler lints (warnings/errors).
- `#[deprecated]` to mark items as outdated.
- `#[must_use]` to warn if the result of a function isn't used.
- `#[cfg(...)]` for conditional compilation.

The following is an example of `#[derive(...)]`:

```rust,editable
// This attribute tells the compiler to automatically generate
// an implementation of the `std::fmt::Debug` trait for the `Point` struct.
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 10, y: 20 };

    // Because we derived `Debug`, we can print the struct using the `{:?}` format specifier.
    // Without `#[derive(Debug)]`, this line would cause a compile-time error.
    println!("The point is: {:?}", p1);
}
```

Attribute can take arguments with different syntaxes:

```rust,compile_fail,noplayground
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Outer attributes `#[attr]` apply to the item below them. Inner attributes `#![attr]` apply to the item that the attribute is declared within. You often see these at the very top of a file (or inline module) to apply to the entire module, or at the very top of `lib.rs` or `main.rs` to apply to the entire crate.

## Mark an Item as `must use` {#must-use}

Apply the `#[must_use]` attribute to functions, methods, or entire types (like structs or enums) to signal that their return value is important and shouldn't be ignored.

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_must_use.rs:example}}
```

Functions that return `Result<T, E>` are often `#[must_use]`. If you call such a function and ignore the `Result`, you're potentially ignoring an error that occurred and could be corrected or reported.

In the builder pattern, methods often return `Self` or a modified version of the builder. The final `.build()` or similar method returns the constructed object. If you forget to call the final method or assign the result, you haven't actually created the object you intended. `#[must_use]` on the builder type or its methods helps catch this.

## Mark an Item as `deprecated` {#deprecated}

When the Rust compiler sees code using an item marked as `#[deprecated]`, it will issue a warning during compilation.

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_deprecated.rs:example}}
```

## Lint Check Attributes {#lint-attributes}

The following attributes are used for controlling diagnostic messages (lints) during compilation.
Replace `...` by the lint name e.g., `dead_code` to detects unused, unexported items.

- `#[allow(...)]` overrides the check, so that violations will go unreported.
- `#[expect(...)]` indicates that lint is expected to be emitted.
- `#[warn(...)]` warns about violations but continues compilation.
- `#[deny(...)]` signals an error after encountering a violation,
- `#[forbid(...)]` is the same as `deny`, but also forbids changing the lint level afterwards.

To print the list of compiler 'lint' options and default settings, enter `rustc -W help` at the command prompt. {{hi:Lint checks}}
[`rustc`][rustc]{{hi:rustc}}⮳ also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`.

You can apply lint attributes to specific items ([functions][p-functions], structs, etc.) or to entire modules:

```rust,editable
{{#include ../../crates/language/tests/attributes/allow_dead_code.rs:example}}
```

See also [Lints (`rustc` book)](https://doc.rust-lang.org/rustc/lints/index.html)⮳.

### Suppress Irrelevant Warnings during Early Development {#early-development-attributes}

During early development, consider placing the following lint attributes at the top of your `main.rs` or `lib.rs`.

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_debug.rs:example}}
```

### Enforce Good Practices and Catch Issues Early {#production-code-attributes}

For production-ready code{{hi:Production-ready code}}, replace the above by the following (or similar):

```rust,editable
{{#include ../../crates/language/tests/attributes/attributes_production.rs:example}}
```

## Compile Conditionally {#conditional-compilation}

Conditional compilation includes or excludes specific pieces of your code based on conditions that are checked at compile time.
Use the `#[cfg(...)]` attribute to write code that only compiles for a specific operating system or architecture (like x86_64, ARM).

```rust,editable
{{#include ../../crates/language/tests/attributes/conditional_compilation.rs:example}}
```

See also [Conditional compilation][book-rust-reference-conditional-compilation]⮳.

### Conditionally compile code blocks with `cfg-if` {#cfg-if}

[![cfg-if][c-cfg_if-badge]][c-cfg_if] [![cfg-if-crates.io][c-cfg_if-crates.io-badge]][c-cfg_if-crates.io] [![cfg-if-github][c-cfg_if-github-badge]][c-cfg_if-github] [![cfg-if-lib.rs][c-cfg_if-lib.rs-badge]][c-cfg_if-lib.rs]{{hi:cfg-if}}

`cfg-if` is a macro to ergonomically define an item depending on a large number of `#[cfg]` parameters. It is structured like an "if-else" chain. The first matching branch is the item that gets emitted.

```rust,editable
{{#include ../../crates/language/tests/attributes/cfg_if.rs:example}}
```

## Automatically Derive Common Traits {#automatic-trait-derivation}

See [Automatic derivation][p-automatic-derivation].

## Related Topics {#skip}

- [[derive | Derive]].
- [[rust-patterns | Rust Patterns]].
- [[testing | Testing]].

## References

- [Attributes reference][book-rust-reference-attributes]⮳.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/985)
automatic derivation location
https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
</div>
