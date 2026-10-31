<!-- TODO Reviewed ??-??-2026 -->

# Rust Scripting

{{#include rhai.incl.md}}

## Embed Rust Scripting in Your Application {#rust-scripting}

{{#badges rhai embedded="Embedded development" parser-implementations="Parser implementations" no-std="No standard library" wasm=WebAssembly}}

[rhai][c~rhai~website]↗{{hi:rhai}} is an embedded scripting language and evaluation engine that gives a safe and easy way to add scripting to any Rust application.{{hi:Scripting in Rust}}

Features:

- Similar to JavaScript + Rust, with dynamic typing.
- Tight integration with native Rust [functions][p~functions] and types.
- Function Registration: it can expose Rust functions into scripts.
- Sandboxing: scripts only access capabilities made available to them, but registered host functions can have side effects. Configure resource limits and expose only the functions and data the script needs.
- Robust error reporting.

```rust,editable
{{#include ../../../crates/other/examples/scripting/rhai.rs:example}}
```

## Expose Rust Types and Sandbox Scripts {#rhai-types-and-limits}

[![rhai][c~rhai~docs~badge]][c~rhai~docs]{{hi:rhai}}

Register a Rust type with `register_type_with_name`, then add a constructor, methods (`register_fn`) and property getters (`register_get`). Reuse a `Scope` across calls to keep script variables between runs.

Untrusted scripts must not run forever. `set_max_operations` and `set_max_call_levels` bound the work and recursion depth, and `disable_symbol("eval")` removes `eval` from the language. A script that exceeds the operation limit fails with `ErrorTooManyOperations`.

```rust,editable
{{#include ../../../crates/other/examples/scripting/rhai_types_and_limits.rs:example}}
```

## Rhai Ecosystem {#rhai-ecosystem}

There are independent packages that provide additional functionality, for example:

- [`rhai-fs`][c~rhai-fs~docs]↗{{hi:rhai-fs}} provides file system access within Rhai scripts.
- [`rhai-rand`][c~rhai-rand~docs]↗{{hi:rhai-rand}} provides random number generation using the [`rand`][c~rand~docs]↗{{hi:rand}} crate and array shuffling and sampling.
- [`rhai-sci`][c~rhai-sci~docs]↗{{hi:rhai-sci}} provides functions useful for scientific computing, inspired by languages like MATLAB, Octave, and R.

## Related Topics {#related-topics .skip}

- [[scripting | Scripting]].
- [[parsing | Parsing]].
- [[embedded | Embedded Systems]].
- [[programming_languages | Programming Languages]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
