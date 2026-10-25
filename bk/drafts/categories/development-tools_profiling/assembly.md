<!-- TODO Reviewed ??-??-2026 -->

# Low-level Profiling Tools

{{#include assembly.incl.md}}

## Inspect the Generated Assembly {#cargo-show-asm}

[![cargo-show-asm][c~cargo-show-asm~docs~badge]][c~cargo-show-asm~docs]{{hi:cargo-show-asm}}
[![cargo-show-asm~crates.io][c~cargo-show-asm~crates.io~badge]][c~cargo-show-asm~crates.io]
[![cargo-show-asm~repo][c~cargo-show-asm~repo~badge]][c~cargo-show-asm~repo]
[![cargo-show-asm~lib.rs][c~cargo-show-asm~lib.rs~badge]][c~cargo-show-asm~lib.rs]

[`cargo-show-asm`][c~cargo-show-asm~repo]↗{{hi:cargo-show-asm}} lets you inspect the compiler output for a Rust function without hand-writing assembly. Install it with `cargo install --locked cargo-show-asm`, then run `cargo asm --help` for the options supported by your installed version. Start with `cargo asm --lib` to list functions from a library crate, or pass a function path such as `cargo asm my_crate::module::function` to inspect a specific item. Use `--bin <name>` or `--example <name>` when the function belongs to a binary or example target.

The tool can display assembly and compiler intermediate representations such as LLVM IR and MIR; consult its help for the current output switches and target options. If you need to compare architecture-specific output, select the same target triple and profile that you measured. A release build may optimize away a function or inline it into its callers, while generic functions can have separate monomorphized instances.

### Use assembly output to answer a focused question

Inspect a function after profiling or benchmarking has identified it as a meaningful cost. Look for concrete evidence such as an unexpected bounds check, repeated conversion, allocation call, branch, or failure to inline. The compiler may have a good reason for the emitted instructions, so change the source only when the output supports a hypothesis and then measure the resulting program again.

`cargo-show-asm` asks the compiler to emit code for selected functions. By default it does not disassemble the final linked executable, and it may not show code compiled earlier inside the standard library or another dependency. Use a disassembler for the final binary when the question concerns linked machine code or dependency implementation.

Generated output depends on the compiler version, target, features, profile, and build flags. Record those inputs when sharing an assembly listing, and avoid treating a listing from one build as a guarantee about another.

## Related Topics {#related-topics .skip}

- [[benchmarking | Benchmarking]] to measure the workload before and after a change.
- [[performance | Cargo Performance Tools]] for build and test workflow tools.
- [[memory_usage_analysis | Memory Usage Analysis]] for heap and process memory profiling.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
