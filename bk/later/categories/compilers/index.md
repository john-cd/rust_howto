# Compilers

Compiler{{hi:Compilers}} implementations, including interpreters{{hi:Interpreters}} and transpilers{{hi:Transpilers}}.

Tools for programming languages: compilers, interpreters, transpilers, and virtual machines.

## Write Compilers

### Incremental Computation

{{#include incremental_computation.incl.md}}

## Compiler Implementations

FIXME

### Interpreters

FIXME

### Transpilers

See also [[transpilers | Transpilers]].

## Linking

`link-cplusplus`: Link libstdc++ or libc++ automatically or manually.

## Code Generation

[`typify`][c~typify~docs]↗{{hi:typify}}: JSON schema to rust type code generator.

## Compiler Implementations, Including Interpreters and Transpilers

### Cucumber

- [`gherkin`][c~gherkin~docs]↗{{hi:gherkin}} is a pure Rust implementation of Gherkin language (`.feature` file) for Cucumber testing framework.
- `cucumber-expressions`: Cucumber Expressions AST and parser.

### Protobuf

- [`protox`][c~protox~docs]↗{{hi:protox}} is a rust implementation of the protobuf compiler.

### Rust

- [`polonius-engine`][c~polonius-engine~docs]↗{{hi:}}: Core definition for the Rust borrow checker.

### C

- [`clang`][c~clang~docs]↗{{hi:clang}}: A somewhat idiomatic Rust wrapper for `libclang` [(clang website)](https://clang.llvm.org)↗{{hi:libclang}}.

### Swift

- [`swift-rs`][c~swift-rs~docs]↗{{hi:swift-rs}}:  Call Swift from Rust with ease!

### JavaScript

- [`oxc`][c~oxc~docs]↗{{hi:oxc}}

See also: [[interfacing_with_javascript | Interfacing With Javascript]].

## Related Topics

- [[development-tools | Development Tools]].
- [[development-tools_build-utils | Development Tools: Build Utils]].
- [[development-tools_cargo-plugins | Development Tools: Cargo Plugins]].
- [[development-tools_debugging | Development Tools: Debugging]].
- [[development-tools_ffi | Development Tools: FFI]] (and [[external-ffi-bindings | External FFI Bindings]]).
- [[development-tools_procedural-macro-helpers | Development Tools: Procedural Macro Helpers]].
- [[development-tools_profiling | Development Tools: Profiling]].
- [[development-tools_testing | Development Tools: Testing]].
- [[parsing | Parsing]], [[parser-implementations | Parser Implementations]], esp. of [[programming_languages | Programming Languages]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/908)
decide what to cover
</div>
