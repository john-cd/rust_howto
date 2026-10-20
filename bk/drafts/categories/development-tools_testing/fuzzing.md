# Fuzzing

{{#include fuzzing.incl.md}}

Fuzzing (or fuzz testing) is an automated technique that involves providing invalid, unexpected, or random data as input to a computer program. The purpose is to find security vulnerabilities, crashes, and undefined behaviors. For example, a web application might be subjected to random strings, files, or network requests to see if any of them cause the application to crash or behave unexpectedly.

Fuzzing is a unguided approach to random [testing][p~testing]. A fuzzer generally provides an input of random bytes, and then examines fairly generic properties (such as "doesn't crash" or "commit undefined behavior") about the resulting program.

Fuzzers generally get their power through a kind of evolutionary algorithm that rewards new mutant inputs that "discover" new branches of the program under test. Fuzzers are excellent for [testing][p~testing] security boundaries, precisely because they make no validity assumptions (hence, they are "unguided") when generating the input.

## Fuzzing Engines {#fuzzing-engines .skip}

- [`cargo fuzz`][c~cargo-fuzz~repo]↗{{hi:cargo fuzz}}: The most common and recommended way to perform fuzzing in Rust. It integrates well with [Cargo][p~cargo] and uses libFuzzer under the hood.
- [`afl.rs`][c~afl~docs]↗{{hi:afl.rs}}: Bindings to the American Fuzzy Lop (AFL) fuzzer. AFL is a powerful fuzzer, but [cargo][p~cargo] fuzz with libFuzzer is often sufficient and easier to set up.
- [`honggfuzz-rs`][c~honggfuzz~repo]↗{{hi:honggfuzz}}: Rust bindings for Honggfuzz, a security-oriented fuzzer with powerful analysis capabilities.
- [`bolero`][c~bolero~repo]↗{{hi:bolero}}: A fuzzing frontend that allows you to write a single fuzz target and run it against multiple engines (libFuzzer, AFL, Honggfuzz) or even as a regular unit test.

In almost all cases, [`cargo-fuzz`][c~cargo-fuzz~docs]↗{{hi:cargo-fuzz}} will be your primary tool for fuzzing Rust code. It uses [`libFuzzer`][libFuzzer~website]↗{{hi:libFuzzer}}, a powerful and modern fuzzing engine, and simplifies the fuzzing process significantly. Make sure to define good fuzz targets and use sanitizers to detect errors effectively.

### Target Definition {#Target Definition .skip}

You'll need to define a fuzz target - a function in your code that the fuzzer will call with different inputs. This is a crucial step in setting up fuzzing.

### Corpus Management {#Corpus Management .skip}

Important for effective fuzzing.

A corpus is a set of initial inputs that the fuzzer uses as a starting point. [`cargo fuzz`][c~cargo-fuzz~repo]↗{{hi:cargo fuzz}} helps manage corpora.

### Coverage-Guided Fuzzing {#Coverage-Guided Fuzzing .skip}

Most effective.

[`libFuzzer`][libFuzzer~website]↗{{hi:libFuzzer}} (used by `cargo fuzz`): libFuzzer is a coverage-guided fuzzer, meaning it uses [code coverage][p~code-coverage] information to guide its search for bugs.
AFL: Also a coverage-guided fuzzer.

### Sanitizers (for Detecting errors) {#Sanitizers (for Detecting errors) .skip}

Address Sanitizer (ASan): Detects memory errors (e.g., use-after-free, memory leaks). Enable it with compiler flags (e.g., `-fsanitize=address`).
Undefined Behavior Sanitizer (UBSan): Detects undefined behavior (e.g., integer overflow, out-of-bounds access). Enable it with compiler flags (e.g., `-fsanitize=undefined`).

### Structured Fuzzing with `arbitrary` {#structured-fuzzing .skip}

While many fuzzers provide raw byte slices, many Rust functions expect structured data. The [`arbitrary`][c~arbitrary~docs]↗{{hi:arbitrary}} crate allows you to automatically generate structured data from raw fuzzer input.

### Differential Fuzzing: (For Comparing implementations) {#Differential Fuzzing: (For Comparing implementations) .skip}

This is a more advanced technique where you provide the same input to two different implementations of the same logic and compare their outputs. This is excellent for finding logic bugs or regressions.

## `cargo fuzz` {#cargo-fuzz}

[![cargo-fuzz][c~cargo-fuzz~docs~badge]][c~cargo-fuzz~docs] [![cargo-fuzz~crates.io][c~cargo-fuzz~crates.io~badge]][c~cargo-fuzz~crates.io] [![cargo-fuzz~repo][c~cargo-fuzz~repo~badge]][c~cargo-fuzz~repo] [![cargo-fuzz~lib.rs][c~cargo-fuzz~lib.rs~badge]][c~cargo-fuzz~lib.rs]{{hi:cargo-fuzz}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

A [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand for fuzzing with [`libFuzzer`][libFuzzer~website]↗{{hi:libFuzzer}}. It is the most integrated tool for Rust fuzzing.

To get started:
1. Install: `cargo install cargo-fuzz`
2. Initialize: `cargo fuzz init`
3. Add your logic to `fuzz/fuzz_targets/target1.rs`
4. Run: `cargo fuzz run target1`

## `afl` {#afl}

[![afl][c~afl~docs~badge]][c~afl~docs] [![afl~crates.io][c~afl~crates.io~badge]][c~afl~crates.io] [![afl~repo][c~afl~repo~badge]][c~afl~repo] [![afl~lib.rs][c~afl~lib.rs~badge]][c~afl~lib.rs]{{hi:afl}}

Fuzz testing (or fuzzing) is a software [testing][p~testing] technique used to find security and stability issues by providing pseudo-random data as input to the software. AFLplusplus is a popular, effective, and modern fuzz [testing][p~testing] tool based on AFL. [`afl.rs`][c~afl~docs]↗{{hi:afl.rs}} allows one to run AFLplusplus on code written in the Rust programming [language][p~language].

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/fuzzing/afl.rs:example}}
```

## `bolero` {#bolero}

[![bolero][c~bolero~docs~badge]][c~bolero~docs] [![bolero~crates.io][c~bolero~crates.io~badge]][c~bolero~crates.io] [![bolero~repo][c~bolero~repo~badge]][c~bolero~repo] [![bolero~lib.rs][c~bolero~lib.rs~badge]][c~bolero~lib.rs]{{hi:bolero}}

[`bolero`][c~bolero~docs]↗{{hi:bolero}} provides a unified interface for multiple fuzzing engines. It is particularly useful because it allows you to run your fuzz targets as regular unit tests during development, ensuring that they don't bit-rot.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/fuzzing/bolero_example.rs:example}}
```

## `honggfuzz` {#honggfuzz}

[![honggfuzz][c~honggfuzz~docs~badge]][c~honggfuzz~docs] [![honggfuzz~crates.io][c~honggfuzz~crates.io~badge]][c~honggfuzz~crates.io] [![honggfuzz~repo][c~honggfuzz~repo~badge]][c~honggfuzz~repo] [![honggfuzz~lib.rs][c~honggfuzz~lib.rs~badge]][c~honggfuzz~lib.rs]{{hi:honggfuzz}}

[`honggfuzz-rs`][c~honggfuzz~docs]↗{{hi:honggfuzz}} provides bindings for Honggfuzz. It is known for its multi-process and multi-threaded design, making it very fast on multi-core systems.

## Memory Testing with Valgrind {#valgrind}

[![valgrind][c~valgrind~docs~badge]][c~valgrind~docs] [![valgrind~crates.io][c~valgrind~crates.io~badge]][c~valgrind~crates.io] [![valgrind~repo][c~valgrind~repo~badge]][c~valgrind~repo] [![valgrind~lib.rs][c~valgrind~lib.rs~badge]][c~valgrind~lib.rs]{{hi:valgrind}}

While not a fuzzer itself, [`valgrind`][c~valgrind~docs]↗{{hi:valgrind}} is an instrumentation framework for building dynamic analysis tools. It is commonly used to detect memory leaks and memory management bugs, which are often what fuzzers are trying to trigger.

## Related Topics {#related-topics .skip}

- [[property_based_testing | Property-Based Testing]]
- [[code_coverage | Code Coverage]]

{{#include refs.incl.md}}
{{#include ../../../src/refs/link-refs.md}}
