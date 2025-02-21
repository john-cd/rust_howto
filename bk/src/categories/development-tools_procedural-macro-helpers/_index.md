# Development Tools: Procedural Macro Helpers

[![cat-development-tools::procedural-macro-helpers][cat-development-tools::procedural-macro-helpers-badge]][cat-development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

Rust macros enable metaprogramming, allowing you to write code that generates other code at compile time.

This capability allows for code reuse by reducing boilerplate and enhancing readability through more expressive syntax. Macros are expanded at compile time, so their output is checked syntactically and type checked. Macros are used for tasks like implementing common traits, creating domain-specific languages, and achieving performance optimizations through compile-time computation and code transformation.

The term 'macro' [(book)](https://doc.rust-lang.org/book/ch19-06-macros.html) refers to a family of features in Rust: declarative macros (also known as "macros by example") and three kinds of _procedural macros_, the focus of this section:

- Custom #[derive] macros, used on structs and enums, which specify code to be added,
- Attribute-like macros that define custom attributes usable on any item,
- Function-like macros that look like function calls but operate on the tokens specified as their argument.

Declarative macros are defined using a `macro_rules!` syntax and work by pattern matching, where you provide patterns and corresponding code templates. They are excellent for simple syntactic transformations and reducing boilerplate based on structural patterns. Procedural macros, on the other hand, are more like functions that operate on Rust code as input and produce Rust code as output. Procedural macros are one of the more complex but powerful parts of Rust, allowing for complex code generation and manipulation of the abstract syntax tree (AST).

## Write procedural macros

{{#include write_proc_macros.incl.md}}

## Tools for macro development

{{#include macro_tools.incl.md}}

## Compile macros ahead of time

{{#include compile_macros.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[procedural-macro-helpers/_index: fix (P2)](https://github.com/john-cd/rust_howto/issues/332)

## Procedural Macro Definition

`proc-macro`: (Standard library crate) Provides the core tools for writing procedural macros, including token streams, parsing, and code generation. This is what you'll use inside your macro definition crate.

Macro Attributes and Derives:

You use `#[derive(...)]` and `#[attribute]` to define and apply macros.

## Parsing

`syn`: A popular crate for parsing Rust code into an Abstract Syntax Tree (AST). This is essential for most procedural macros that need to understand the structure of the code they're manipulating.

Code Generation:

`quote`: A crate that makes it easy to generate Rust code (as TokenStreams). It provides quasiquoting, which makes code generation much more readable and less error-prone.
Testing:

`trybuild`: A crate that simplifies testing procedural macros by allowing you to write test cases that check if your macro generates the expected code and handles errors correctly.
Debugging:

Debugging procedural macros can be tricky. Often involves printing token streams to the console or using tools like cargo expand to see the generated code.

## Metaprogramming Concepts

Understanding how macros work, token streams, ASTs, and the difference between attribute macros, derive macros, and function-like macros is crucial.

## Documentation

Good documentation is essential for any procedural macro. Document your macro's usage, attributes, and any potential pitfalls.

In short, you'll almost always use `proc-macro`, `syn`, and `quote` when writing procedural macros. `trybuild` is highly recommended for testing. Understanding the underlying metaprogramming concepts is key to effectively using these crates.

</div>
