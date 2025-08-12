# Fuzzing

{{#include fuzzing.incl.md}}

Fuzzing (or fuzz testing) is an automated technique that involves providing invalid, unexpected, or random data as input to a computer program. The purpose is to find security vulnerabilities, crashes, and undefined behaviors. For example, a web application might be subjected to random strings, files, or network requests to see if any of them cause the application to crash or behave unexpectedly.

Fuzzing is a unguided approach to random [testing][p~testing]. A fuzzer generally provides an input of random bytes, and then examines fairly generic properties (such as "doesn't crash" or "commit undefined behavior") about the resulting program.

Fuzzers generally get their power through a kind of evolutionary algorithm that rewards new mutant inputs that "discover" new branches of the program under test. Fuzzers are excellent for [testing][p~testing] security boundaries, precisely because they make no validity assumptions (hence, they are "unguided") when generating the input.

## Fuzzing Engines {#skip}

- [`cargo fuzz`][c~cargo-fuzz~github]↗{{hi:cargo fuzz}}: The most common and recommended way to perform fuzzing in Rust. It integrates well with [Cargo][p~cargo] and uses libFuzzer under the hood.
- [`afl.rs`][c~afl~docs]↗{{hi:afl.rs}}: Bindings to the American Fuzzy Lop (AFL) fuzzer. AFL is a powerful fuzzer, but [cargo][p~cargo] fuzz with libFuzzer is often sufficient and easier to set up.

In almost all cases, [`cargo fuzz`]( ){{hi: }} will be your primary tool for fuzzing Rust code. It uses [`libFuzzer`](https://llvm.org/docs/LibFuzzer.html)↗{{hi:libFuzzer}}, a powerful and modern fuzzing engine, and simplifies the fuzzing process significantly. Make sure you define good fuzz targets and use sanitizers to detect errors effectively.

### Target Definition {#skip}

You'll need to define a fuzz target - a function in your code that the fuzzer will call with different inputs. This is a crucial step in setting up fuzzing.

### Corpus Management {#skip}

(Important for effective fuzzing)

A corpus is a set of initial inputs that the fuzzer uses as a starting point. [`cargo fuzz`][c~cargo-fuzz~github]↗{{hi:cargo fuzz}} helps manage corpora.

### Coverage-Guided Fuzzing {#skip}

(Most effective)

[`libFuzzer`](https://llvm.org/docs/LibFuzzer.html)↗{{hi:libFuzzer}} (used by `cargo fuzz`): libFuzzer is a coverage-guided fuzzer, meaning it uses [code coverage][p~code-coverage] information to guide its search for bugs.
AFL: Also a coverage-guided fuzzer.

### Sanitizers (for Detecting errors) {#skip}

Address Sanitizer (ASan): Detects memory errors (e.g., use-after-free, memory leaks). Enable it with compiler flags (e.g., `-fsanitize=address`).
Undefined Behavior Sanitizer (UBSan): Detects undefined behavior (e.g., integer overflow, out-of-bounds access). Enable it with compiler flags (e.g., `-fsanitize=undefined`).

### Differential Fuzzing: (For Comparing implementations) {#skip}

This is a more advanced technique and often involves custom [scripting][p~scripting] or tools. There aren't specific crates for it.

## `cargo fuzz` {#cargo-fuzz}

[![cargo-fuzz][c~cargo-fuzz~docs~badge]][c~cargo-fuzz~docs] [![cargo-fuzz~crates.io][c~cargo-fuzz~crates.io~badge]][c~cargo-fuzz~crates.io] [![cargo-fuzz~github][c~cargo-fuzz~github~badge]][c~cargo-fuzz~github] [![cargo-fuzz~lib.rs][c~cargo-fuzz~lib.rs~badge]][c~cargo-fuzz~lib.rs]{{hi:cargo-fuzz}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}}

A [`cargo`][c~cargo~docs]↗{{hi:cargo}} subcommand for fuzzing with [`libFuzzer`](https://llvm.org/docs/LibFuzzer.html)↗{{hi:libFuzzer}}.

## `afl` {#afl}

[![afl][c~afl~docs~badge]][c~afl~docs] [![afl~crates.io][c~afl~crates.io~badge]][c~afl~crates.io] [![afl~github][c~afl~github~badge]][c~afl~github] [![afl~lib.rs][c~afl~lib.rs~badge]][c~afl~lib.rs]{{hi:afl}}

Fuzz testing (or fuzzing) is a software [testing][p~testing] technique used to find security and stability issues by providing pseudo-random data as input to the software. AFLplusplus is a popular, effective, and modern fuzz [testing][p~testing] tool based on AFL. [`afl.rs`][c~afl~docs]↗{{hi:afl.rs}} allows one to run AFLplusplus on code written in the Rust programming [language][p~language].

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/examples/fuzzing/afl.rs:example}}
```

## Related Topic: Property-Based Testing {#skip}

The [`proptest`][c~proptest~docs]↗{{hi:proptest}}, [`quickcheck`][c~quickcheck~docs]↗{{hi:quickcheck}} crates are used for property-based testing, which is a different but complementary technique to fuzzing. Property-based testing generates many random inputs to verify properties of your code.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fuzzing: review fuzzing crates](https://github.com/john-cd/rust_howto/issues/339)
cover [bolero][bolero~github]↗.
[`bolero`][c~bolero~docs]↗{{hi:bolero}}
cover valgrind
[`valgrind`][c~valgrind~docs]↗{{hi:valgrind}}.
</div>
