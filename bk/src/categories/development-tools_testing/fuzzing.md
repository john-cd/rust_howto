<!-- TODO Reviewed ??-??-2026. -->

# Fuzzing

{{#include fuzzing.incl.md}}

Fuzzing (or fuzz testing) repeatedly supplies varied inputs to a program and checks for failures such as crashes, hangs, failed assertions, or sanitizer reports. It can uncover security bugs, but finding them depends on the target, input model, and available instrumentation.

An unguided fuzzer starts with arbitrary bytes and checks general properties such as "does not crash." Coverage-guided fuzzers use execution feedback to find inputs that reach new code.

Coverage-guided fuzzers mutate inputs that reach new branches, then keep useful cases in a corpus. This is valuable for parsers and other security boundaries where valid-input assumptions can hide edge cases.

## Fuzzing Engines {#fuzzing-engines .skip}

- [`cargo fuzz`][c~cargo-fuzz~repo]↗{{hi:cargo fuzz}} is a common Rust interface to libFuzzer and integrates with Cargo.
- [`afl`][c~afl~docs]↗{{hi:afl}} provides bindings for AFL++. For a first Rust fuzz target, `cargo fuzz` with libFuzzer is often simpler to set up.
- [`honggfuzz-rs`][c~honggfuzz~repo]↗{{hi:honggfuzz}}: Rust bindings for Honggfuzz, a security-oriented fuzzer with powerful analysis capabilities.
- [`bolero`][c~bolero~repo]↗{{hi:bolero}}: A fuzzing frontend that allows you to write a single fuzz target and run it against multiple engines (libFuzzer, AFL, Honggfuzz) or even as a regular unit test.

In almost all cases, [`cargo-fuzz`][c~cargo-fuzz~docs]↗{{hi:cargo-fuzz}} will be your primary tool for fuzzing Rust code. It uses [`libFuzzer`][libFuzzer~website]↗{{hi:libFuzzer}}, a powerful and modern fuzzing engine, and simplifies the fuzzing process significantly. Make sure to define good fuzz targets and use sanitizers to detect errors effectively.

### Target Definition {#target-definition .skip}

You'll need to define a fuzz target - a function in your code that the fuzzer will call with different inputs. This is a crucial step in setting up fuzzing.

### Corpus Management {#corpus-management .skip}

Important for effective fuzzing.

A corpus is a set of initial inputs that the fuzzer uses as a starting point. [`cargo fuzz`][c~cargo-fuzz~repo]↗{{hi:cargo fuzz}} helps manage corpora.

### Coverage-Guided Fuzzing {#coverage-guided-fuzzing .skip}

Most effective.

[`libFuzzer`][libFuzzer~website]↗{{hi:libFuzzer}} (used by `cargo fuzz`) is coverage-guided: it uses [code coverage](code_coverage.md) feedback to find inputs that reach new paths.
AFL: Also a coverage-guided fuzzer.

### Sanitizers for Detecting Errors {#sanitizers .skip}

Sanitizer support depends on the Rust toolchain, target, and fuzzer. Follow the selected tool's Rust-specific setup instructions; C and C++ flags such as `-fsanitize=address` are not Rust compiler options.

### Structured Fuzzing with `arbitrary` {#structured-fuzzing .skip}

While many fuzzers provide raw byte slices, many Rust functions expect structured data. The [`arbitrary`][c~arbitrary~docs]↗{{hi:arbitrary}} crate allows you to automatically generate structured data from raw fuzzer input.

### Differential Fuzzing {#differential-fuzzing .skip}

This is a more advanced technique where you provide the same input to two different implementations of the same logic and compare their outputs. This is excellent for finding logic bugs or regressions.

## `cargo fuzz` {#cargo-fuzz}

{{#badges cargo-fuzz development-tools::testing=Testing}}

A [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand for fuzzing with [`libFuzzer`][libFuzzer~website]↗{{hi:libFuzzer}}. It uses nightly Rust and is a common starting point for Rust fuzz targets.

To get started:

1. Install: `cargo install cargo-fuzz --locked`
2. Initialize: `cargo fuzz init`
3. Adapt the generated target in `fuzz/fuzz_targets/` to call the code under test.
4. Run the generated `fuzz_target_1` target for a bounded interval: `cargo fuzz run fuzz_target_1 -- -max_total_time=30`.

## `afl` {#afl}

{{#badges afl}}

Fuzz testing provides pseudo-random input to find crashes and stability issues. AFL++ is a coverage-guided fuzzer; [`afl`][c~afl~docs]↗{{hi:afl}} provides Rust bindings for writing and running AFL++ targets.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/fuzzing/afl.rs:example}}
```

## `bolero` {#bolero}

{{#badges bolero}}

[`bolero`][c~bolero~docs]↗{{hi:bolero}} provides a unified interface for multiple fuzzing engines. It is particularly useful because it allows you to run your fuzz targets as regular unit tests during development, ensuring that they don't bit-rot.

Test this Cargo-backed example from `bk/crates` with `cargo test --locked -p development_tools_testing --example fuzzing --features development_tools_testing/afl-fuzzing`.

```rust,editable,noplayground
{{#include ../../../crates/cats/development_tools_testing/examples/fuzzing/bolero_example.rs:example}}
```

## `honggfuzz` {#honggfuzz}

{{#badges honggfuzz}}

[`honggfuzz-rs`][c~honggfuzz~docs]↗{{hi:honggfuzz}} provides bindings for Honggfuzz. It is known for its multi-process and multi-threaded design, making it very fast on multi-core systems.

## Memory Testing with Valgrind {#valgrind}

[![valgrind][c~valgrind~docs~badge]][c~valgrind~docs] [![valgrind~crates.io][c~valgrind~crates.io~badge]][c~valgrind~crates.io]{{hi:valgrind}}

[Valgrind's Memcheck manual](https://valgrind.org/docs/manual/mc-manual.html) explains how to detect invalid memory access and leaks in a running program. You can pass a reproducible input found by a fuzzer to the program under Memcheck to investigate its memory behavior.

## Related Topics {#related-topics .skip}

- [Property-Based Testing](property_based_testing.md).
- [Code Coverage](code_coverage.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
