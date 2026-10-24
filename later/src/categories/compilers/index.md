<!-- TODO Reviewed ??-??-2026 -->
# Compilers

This category covers libraries for building compilers, interpreters, and transpilers in Rust. It focuses on language implementation rather than the internals of `rustc`.

## Compiler and Interpreter Tasks

| Task | Crates | When to use them |
|---|---|---|
| Lexing | [logos][c~logos~docs]↗{{hi:logos}} | Generate a lexer from token definitions. The [tokenization example][ex~compilers~tokenization] demonstrates a small lexer. |
| Parsing | [chumsky][c~chumsky~docs]↗{{hi:chumsky}}, [lalrpop][c~lalrpop~docs]↗{{hi:lalrpop}}, [pest][c~pest~docs]↗{{hi:pest}} | Choose parser combinators or a grammar-based parser; see [[parser-implementations | Parser Implementations]]. |
| Diagnostics | [ariadne][c~ariadne~docs]↗{{hi:ariadne}}, [codespan-reporting][c~codespan-reporting~docs]↗{{hi:codespan-reporting}}, [miette][c~miette~docs]↗{{hi:miette}} | Attach source spans and render helpful error reports. |
| Names and allocation | [string-interner][c~string-interner~docs]↗{{hi:string-interner}}, [bumpalo][c~bumpalo~docs]↗{{hi:bumpalo}} | Intern repeated names or allocate related compiler data in an arena. |
| Code generation | [cranelift][c~cranelift~docs]↗{{hi:cranelift}}, [inkwell][c~inkwell~docs]↗{{hi:inkwell}}, [wasm-encoder][c~wasm-encoder~docs]↗{{hi:wasm-encoder}} | Generate machine code, LLVM IR, or WebAssembly. |
| Incremental computation | [salsa][c~salsa~docs]↗{{hi:salsa}}, [comemo][c~comemo~docs]↗{{hi:comemo}} | Reuse work when only part of a program changes. |

Parser choice depends on the grammar and desired error recovery. The [[parsing | Parsing]] chapter covers the concepts and links to more implementations.

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
- [[development-tools_ffi | Development Tools: FFI]] for bindings to compilers written in other languages.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[content audit: category index content removed by recent commits; review and restore](https://github.com/john-cd/rust_howto/issues/1903)
</div>
