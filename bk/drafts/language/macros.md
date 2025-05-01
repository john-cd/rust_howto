# Macros

{{#include macros.incl.md}}

## Macro Syntax {#skip}

[![Rust by example - macros][book-rust-by-example-macros-badge]][book-rust-by-example-macros]{{hi:Macros}}

Macros are a way of writing code that writes other code, known as metaprogramming{{hi:Metaprogramming}}. They allow you to define reusable code snippets that can be expanded into more complex code at compile time. Macros are more flexible than functions because they can handle a variable number of arguments and can manipulate code structure directly.

## Macros By Example {#macros-by-example}

The `macro_rules!` macro is used to define a macro by example. The syntax consists of a name for the macro (e.g., `create_function`), followed by a set of rules enclosed in curly braces `{}`. Each rule has a pattern and a corresponding code block that will be substituted when the macro is invoked.

In the following example, the pattern `($func_name:ident)` matches an identifier (like a function name), and the code block defines a function with that name. The `$func_name` is a macro variable that will be replaced with the actual identifier provided when the macro is called.

## Macro Uses {#macro-uses}

The following example demonstrates various ways macros can be used in Rust.

```rust,editable
{{#include ../../crates/language/tests/macros/macros.rs:example}}
```

## References {#skip}

- [Rust reference - Macros][book-rust-reference-macros]⮳.
- [Rust by example - Macros][book-rust-by-example-macros]⮳ [![Rust by example - macros][book-rust-by-example-macros-badge]][book-rust-by-example-macros]{{hi:Macros}}⮳.
- The [Little Book of Rust Macros][book-rust-macros]⮳.

## Related Topics {#skip}

- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].
  - [[compile_macros | Compile Macros]].
  - [[macro_tools | Macro Tools]].
  - [[write_proc_macros | Write Proc Macros]].
- [[rust-patterns | Rust Patterns]].

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
[macros: write NOW](https://github.com/john-cd/rust_howto/issues/548)
</div>
