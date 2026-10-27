<!-- TODO Reviewed ??-??-2026 -->

# Compilers

This category covers libraries for building compilers, interpreters, and transpilers in Rust. It focuses on language implementation rather than the internals of `rustc`.

## Compiler and Interpreter Tasks

| Task                      | Crates                                                                                                                                                                                                                         | When to use them                                                                                                             |
| ------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| Lexing                    | [logos][c~logos~docs]↗{{hi:logos}}                                                                                                                                                                                             | Generate a lexer from token definitions. The [tokenization example][ex~compilers~tokenization] demonstrates a small lexer.   |
| Parsing                   | [chumsky][c~chumsky~docs]↗{{hi:chumsky}}, [lalrpop][c~lalrpop~docs]↗{{hi:lalrpop}}, [pest][c~pest~docs]↗{{hi:pest}}                                                                                                            | Choose parser combinators or a grammar-based parser; see [[parser-implementations                                            | Parser Implementations]]. |
| Diagnostics               | [ariadne][c~ariadne~docs]↗{{hi:ariadne}}, [codespan-reporting][c~codespan-reporting~docs]↗{{hi:codespan-reporting}}, [miette][c~miette~docs]↗{{hi:miette}}                                                                     | Attach source spans and render helpful error reports.                                                                        |
| Names and allocation      | [string-interner][c~string-interner~docs]↗{{hi:string-interner}}, [bumpalo][c~bumpalo~docs]↗{{hi:bumpalo}}                                                                                                                     | Intern repeated names or allocate related compiler data in an arena.                                                         |
| Code generation           | [cranelift][c~cranelift~docs]↗{{hi:cranelift}}, [inkwell][c~inkwell~docs]↗{{hi:inkwell}}, [wasm-encoder][c~wasm-encoder~docs]↗{{hi:wasm-encoder}}                                                                              | Generate machine code, LLVM IR, or WebAssembly.                                                                              |
| Incremental computation   | [salsa][c~salsa~docs]↗{{hi:salsa}}, [comemo][c~comemo~docs]↗{{hi:comemo}}                                                                                                                                                      | Reuse work when only part of a program changes.                                                                              |
| Schema and protocol tools | [typify][c~typify~docs]↗{{hi:typify}}, [protox][c~protox~docs]↗{{hi:protox}}                                                                                                                                                   | Generate Rust types from JSON Schema or compile Protocol Buffers.                                                            |
| Foreign-language bindings | [link-cplusplus][c~link-cplusplus~crates.io]↗{{hi:link-cplusplus}}, [clang][c~clang~docs]↗{{hi:clang}}, [swift-rs][c~swift-rs~docs]↗{{hi:swift-rs}}                                                                            | Link C++ libraries, use libclang, or call Swift from Rust; these integrations may require native toolchains.                 |
| Language and test support | [gherkin][c~gherkin~docs]↗{{hi:gherkin}}, [cucumber-expressions][c~cucumber-expressions~crates.io]↗{{hi:cucumber-expressions}}, [polonius-engine][c~polonius-engine~docs]↗{{hi:polonius-engine}}, [oxc][c~oxc~docs]↗{{hi:oxc}} | Parse Gherkin and Cucumber expressions, experiment with borrow-checking analysis, or work with the Oxc JavaScript toolchain. |

An interpreter executes source code or bytecode. A transpiler translates source from one language or representation to another. Parser choice depends on the grammar and desired error recovery; the [[parsing | Parsing]] chapter covers the concepts and links to more implementations.

## Examples

{{#include tokenization.incl.md}}
{{#include incremental_computation.incl.md}}

## Learn More

- [Writing Interpreters in Rust][writing-interpreters-rust~website]↗ walks through language implementation in Rust.
- [Crafting Interpreters][crafting-interpreters~website]↗ includes a [catalog of Lox implementations][crafting-interpreters~ports~wiki]↗ with Rust ports.

## Related Topics

- [[transpilers | Transpilers]].
- [[parsing | Parsing]] and [[parser-implementations | Parser Implementations]].
- [[programming_languages | Programming Languages]].
- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_debugging | Development Tools: Debugging]].
- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].
- [[development-tools_profiling | Development Tools: Profiling]].
- [[development-tools_testing | Development Tools: Testing]].
- [[development-tools_ffi | Development Tools: FFI]] for bindings to compilers written in other languages.
- [[interfacing_with_javascript | Interfacing with JavaScript]] for Rust tools that interface with JavaScript.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  content audit: category index content removed by recent commits; review and restore
</div>
