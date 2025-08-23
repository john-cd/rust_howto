# Attributes

{{#include attributes.incl.md}}

## Attribute Syntax {#attribute}

[![Rust by example - attributes][book~rust-by-example~attributes~badge]][book~rust-by-example~attributes]{{hi:Attributes}}

Attributes are annotations you attach to your Rust code, like functions, structs, modules, or even entire crates. They provide extra information or instructions to the Rust compiler or other tools (like linters or documentation generators). They don't change the logic of the code directly, but they influence how it's compiled, checked, or processed.

Common attributes include:

- `#[allow(...)]`, `#[warn(...)]`, `#[deny(...)]` to control compiler lints (ignore them; or make them into warnings or errors).
- `#[deprecated]` to mark items as outdated.
- `#[must_use]` to warn if the result of a function is not used.
- `#[cfg(...)]` for conditional compilation.
- `#[derive(...)]` to autogenerate an implementation of certain traits.
- `#[test]` to mark functions as tests.

Attribute can take arguments with different syntaxes:

```rust,compile_fail,noplayground
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

The following is an example of `#[derive(...)]`:

```rust,editable
{{#include ../../crates/language/examples/attributes/attributes_derive.rs:example}}
```

Outer attributes `#[attr]` apply to the item below them. Inner attributes `#![attr]` apply to the item that the attribute is declared within. You often see these at the very top of a file (or inline module) to apply to the entire module, or at the very top of `lib.rs` or `main.rs` to apply to the entire crate:

```rust,noplayground
// For example, add the following to the top of your file
// to warn if public items are not documented.
#![warn(missing_docs)]
```

## Mark an Item as `must_use` {#must-use}

Apply the `#[must_use]` attribute to functions, methods, or entire types (like structs or enums) to signal that their return value is important and shouldn't be ignored.

```rust,editable
{{#include ../../crates/language/examples/attributes/attributes_must_use.rs:example}}
```

Functions that return `Result<T, E>` are often `#[must_use]`. If you call such a function and ignore the `Result`, you're potentially ignoring an error that occurred and could be corrected or reported.

In the [[builder_pattern | builder pattern]], methods often return `Self` or a modified version of the builder. The final `.build()` or similar method returns the constructed object. If you forget to call the final method or assign the result, you haven't actually created the object you intended. Adding `#[must_use]` on the builder type or its methods helps catch this problem.

## Mark an Item as `deprecated` {#deprecated}

When the Rust compiler sees code using an item marked as `#[deprecated]`, it will issue a warning during compilation:

```rust,editable
{{#include ../../crates/language/examples/attributes/attributes_deprecated.rs:example}}
```

## Control Compilation Diagnostic Messages with Lint Check Attributes {#lint-attributes}

The Rust compiler runs a number of code lints (code diagnostics) when it compiles your code. These lints may produce a warning, an error, or nothing at all, depending on their default lint level and additional attributes you provide, replacing `...` below by the lint name:

- `#[allow(...)]` overrides the check, so that violations will go unreported,
- `#[expect(...)]` indicates that lint is expected to be emitted,
- `#[warn(...)]` warns about violations but continues compilation,
- `#[deny(...)]` signals an error after encountering a violation,
- `#[forbid(...)]` is the same as `deny`, but also forbids changing the lint level afterwards.

You can apply these lint attributes to specific items ([functions][p~functions], structs, etc.) or to entire modules (by using an inner attribute):

```rust,editable
{{#include ../../crates/language/examples/attributes/allow_dead_code.rs:example}}
```

Common lint attributes are:

```rust,noplayground
#[allow(dead_code)]        // Silence warnings about unused functions, methods, or other items.
#[allow(unused_imports)]   // Silence warnings about `use` statements that import items not used in the scope.
#[allow(unused_variables)] // Silence warnings about unused variables in a function.
#![warn(missing_docs)]     // Warn if public items are not documented.
#![deny(unsafe_code)]      // Prevent the use of unsafe blocks.
```

To print the list of compiler 'lint' options and default settings, enter `rustc -W help` at the command prompt, or refer to the [lints chapter (`rustc` book)][book~rustc~lints]↗.{{hi:Lint checks}}

There are also lints for other tools, such as "clippy" and "rustdoc". For example, `#![deny(clippy::all)]` is a very common attribute to make all Clippy lints that are "warn" by default into hard errors. This makes Clippy much stricter. You may also use `#![warn(clippy::pedantic)]` for even more pedantic Clippy lints.

### Suppress Irrelevant Warnings during Early Development {#early-development-attributes}

During early development, consider placing the following lint attributes at the top of your `main.rs` or `lib.rs` file, in order to temporarily ignore distracting warnings:

```rust,editable
{{#include ../../crates/language/examples/attributes/attributes_early_development.rs:example}}
```

### Enforce Good Practices and Catch Issues Early with Lint Attributes {#production-code-attributes}

For production-ready code{{hi:Production-ready code}}, replace the above by the following (or similar) to make linting stricter:

```rust,editable
{{#include ../../crates/language/examples/attributes/attributes_production.rs:example}}
```

## Compile Code Conditionally with `#[cfg(...)]` {#conditional-compilation}

Conditional compilation includes or excludes specific pieces of your code based on conditions that are checked at compile time.
Use the `#[cfg(...)]` attribute to write code that only compiles for a specific operating system or architecture (like x86_64, ARM).

```rust,editable
{{#include ../../crates/language/examples/attributes/conditional_compilation.rs:example}}
```

See also the [conditional compilation][book~rust-reference~conditional-compilation]↗ chapter of the Rust Reference.

### Conditionally Compile Code Blocks with `cfg-if` {#cfg-if}

[![cfg-if][c~cfg-if~docs~badge]][c~cfg-if~docs] [![cfg-if~crates.io][c~cfg-if~crates.io~badge]][c~cfg-if~crates.io] [![cfg-if~repo][c~cfg-if~repo~badge]][c~cfg-if~repo] [![cfg-if~lib.rs][c~cfg-if~lib.rs~badge]][c~cfg-if~lib.rs]{{hi:cfg-if}}

For complicated conditional compilation scenarios, consider using the `cfg-if` crate in addition to the `#[cfg(...)]` attribute.

`cfg-if` is a macro to ergonomically define an item depending on a large number of `#[cfg]` parameters. It is structured like an "if-else" chain. The first matching branch is the item that gets emitted.

```rust,editable
{{#include ../../crates/language/examples/attributes/cfg_if.rs:example}}
```

## Related Topics {#related-topics}

- [[derive | Derive]] for the `#[derive(...)]` attribute used to automatically derive common traits.
- [[rust-patterns | Rust Patterns]].
- [[testing | Testing]] for the `#[test]` attribute.
- [[documentation | Documentation]] for the `#[doc(...)]` attribute.

## References {#references}

- [Attributes reference][book~rust-reference~attributes]↗.
- [The `#[doc]` attribute][book~rustdoc~doc-attribute]↗.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
