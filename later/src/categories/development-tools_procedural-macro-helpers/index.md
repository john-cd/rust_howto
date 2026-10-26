<!-- TODO Reviewed ??-??-2026 -->

# Development Tools: Procedural Macro Helpers

[![cat~development-tools::procedural-macro-helpers][cat~development-tools::procedural-macro-helpers~badge]][cat~development-tools::procedural-macro-helpers]{{hi:Procedural macro helpers}}

Procedural macros run at compile time and transform token streams into generated code. A [`TokenStream`][ex~development-tools_procedural-macro-helpers~proc-macro2] is their input and output format; `proc-macro2` also makes token handling testable outside the compiler's macro interface.

## Write Procedural Macros

[Write Procedural Macros guide][p~write-proc-macros]

{{#include write_proc_macros.incl.md}}

## Tools for Macro Development

[Macro Tools guide][p~macro-tools]

{{#include macro_tools.incl.md}}

## Compile Procedural Macros Ahead of Time

[Compile Macros guide][p~compile-macros]

{{#include compile_macros.incl.md}}

## Related Topics

- [Declarative macros][p~macros].
- [Procedural macro authoring][p~write-proc-macros].
- [Compile-time code generation][p~compile-macros].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
