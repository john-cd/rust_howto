# Development Tools: Procedural Macro Helpers

[![cat~development-tools::procedural-macro-helpers][cat~development-tools::procedural-macro-helpers~badge]][cat~development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

The term 'macro' [(book)][book~rust~macros]↗ refers to a family of features in Rust: declarative macros (also known as "macros by example") and three kinds of _procedural macros_, the focus of this section:

- Custom #[derive] macros, used on structs and enums, which specify code to be added.
- Attribute-like macros that define custom attributes usable on any item.
- Function-like macros that look like function calls but operate on the tokens specified as their argument.

## Write Procedural Macros

{{#include write_proc_macros.incl.md}}

## Tools for Macro Development

{{#include macro_tools.incl.md}}

## Compile Procedural Macros Ahead of Time

{{#include compile_macros.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fix](https://github.com/john-cd/rust_howto/issues/332)
explain:
- token streams.
- ASTs.
</div>
