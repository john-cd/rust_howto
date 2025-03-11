# Development Tools: Procedural Macro Helpers

[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

Rust macros enable metaprogramming, allowing you to write code that generates other code at compile time.

This capability allows for code reuse by reducing boilerplate and enhancing readability through more expressive syntax. Macros are expanded at compile time, so their output is checked syntactically and type checked. Macros are used for tasks like implementing common traits, creating domain-specific languages, and achieving performance optimizations through compile-time computation and code transformation.

The term 'macro' [(book)][book-macro] refers to a family of features in rust: declarative macros (also known as "macros by example")
 and three kinds of _procedural macros_, the focus of this section:

- Custom #[derive] macros, used on structs and enums, which specify code to be added.
- Attribute-like macros that define custom attributes usable on any item.
- Function-like macros that look like function calls but operate on the tokens specified as their argument.

Declarative macros are defined using a `macro_rules!` syntax and work by pattern matching, where you provide patterns and corresponding code templates. They are excellent for simple syntactic transformations and reducing boilerplate based on structural patterns. Procedural macros, on the other hand, are more like functions that operate on Rust code as input and produce Rust code as output. Procedural macros are one of the more complex but powerful parts of Rust, allowing for complex code generation and manipulation of the abstract syntax tree (AST).

## Write procedural macros

{{#include write_proc_macros.incl.md}}

## Tools for macro development

{{#include macro_tools.incl.md}}

## Compile macros ahead of time

{{#include compile_macros.incl.md}}

[book-macro]: https://doc.rust-lang.org/book/ch19-06-macros.html
{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix](https://github.com/john-cd/rust_howto/issues/332)
explain
- token streams.
- ASTs.
</div>
