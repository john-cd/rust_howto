<!-- TODO Reviewed ??-??-2026 -->

# Transpilers

{{#include transpilers.incl.md}}

## Translate C to Rust with C2Rust {#c2rust}

[![c2rust~website][c~c2rust~website~badge]][c~c2rust~website] {{#badges c2rust}} {{hi:Migration}} {{hi:Translation}} {{hi:Transpiler}} {{hi:C}} {{#categories development-tools="Development tools",development-tools::ffi=FFI,command-line-utilities="Command line utilities"}}

C2Rust uses Clang's compilation database to translate C99 projects into Rust. Build and test the original project first so you can compare the translated code against known behavior. The generated Rust mirrors the C structure and can contain substantial `unsafe`; treat it as a migration starting point, then refactor and verify it rather than assuming translation made it memory-safe.

The following commands assume a CMake project and a C2Rust installation on a supported macOS or Linux host. C2Rust also documents alternatives for other build systems that can produce `compile_commands.json`.

```sh
# Install C2Rust with the lockfile published by the project.
cargo install --locked c2rust

# Configure the C project and emit the exact compiler commands used to build it.
cmake -S c-project -B c-project/build -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

# Translate from the compilation database; use a scratch checkout for generated files.
c2rust transpile --emit-build-files c-project/build/compile_commands.json
```

The compilation database must be named `compile_commands.json` and include the compiler flags and include paths used by the original build. Review warnings and skipped definitions, compile the generated crate, run the original tests or cross-checking workflow, then replace unsafe interfaces incrementally.

## Related Topics {#related-topics .skip}

- [[development-tools_ffi | Development Tools: FFI]].
- [[wasm | WebAssembly]].

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
