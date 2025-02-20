# Fuzzing

{{#include fuzzing.incl.md}}

## `afl` {#afl}

[![afl][c-afl-badge]][c-afl] [![afl-crates.io][c-afl-crates.io-badge]][c-afl-crates.io] [![afl-github][c-afl-github-badge]][c-afl-github] [![afl-lib.rs][c-afl-lib.rs-badge]][c-afl-lib.rs]{{hi:afl}}

Fuzz testing (or fuzzing) is a software [testing][p-testing] technique used to find security and stability issues by providing pseudo-random data as input to the software. AFLplusplus is a popular, effective, and modern fuzz [testing][p-testing] tool based on AFL. `afl.rs` allows one to run AFLplusplus on code written in the Rust programming [language][p-language].

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/afl.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fuzzing: review fuzzing crates (P1)](https://github.com/john-cd/rust_howto/issues/339)

Fuzzing (for example, with cargo-fuzz) is a unguided approach to random [testing][p-testing]. A fuzzer generally provides an input of random bytes, and then examines fairly generic properties (such as "doesn't crash" or "commit undefined behavior") about the resulting program.

Fuzzers generally get their power through a kind of evolutionary algorithm that rewards new mutant inputs that "discover" new branches of the program under test. Fuzzers are excellent for [testing][p-testing] security boundaries, precisely because they make no validity assumptions (hence, they are "unguided") when generating the input.

TODO P2 cover [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)

TODO P2 cover property [testing][p-testing] [proptest](https://github.com/proptest-rs/proptest)

```toml
[dev-dependencies]
proptest = "1.0.0"
```

TODO P2 cover [bolero](https://github.com/camshaft/bolero/)

`bolero`

TODO P2 cover valgrind

`valgrind`

## Fuzzing Engines

cargo fuzz: The most common and recommended way to perform fuzzing in Rust. It integrates well with Cargo and uses libFuzzer under the hood.
afl.rs: Bindings to the American Fuzzy Lop (AFL) fuzzer. AFL is a powerful fuzzer, but cargo fuzz with libFuzzer is often sufficient and easier to set up.

## Target Definition: (Essential)

You'll need to define a fuzz target—a function in your code that the fuzzer will call with different inputs. This is a crucial step in setting up fuzzing.

## Corpus Management: (Important for effective fuzzing)

A corpus is a set of initial inputs that the fuzzer uses as a starting point. cargo fuzz helps manage corpora.

## Coverage-Guided Fuzzing: (Most effective)

libFuzzer (used by cargo fuzz): libFuzzer is a coverage-guided fuzzer, meaning it uses code coverage information to guide its search for bugs.
AFL: Also a coverage-guided fuzzer.

## Sanitizers (for detecting errors)

Address Sanitizer (ASan): Detects memory errors (e.g., use-after-free, memory leaks). Enable it with compiler flags (e.g., -fsanitize=address).
Undefined Behavior Sanitizer (UBSan): Detects undefined behavior (e.g., integer overflow, out-of-bounds access). Enable it with compiler flags (e.g., -fsanitize=undefined).

## Differential Fuzzing: (For comparing implementations)

This is a more advanced technique and often involves custom scripting or tools. There aren't specific crates for it.

## Property-Based Testing (Related, but not strictly fuzzing)

proptest, quickcheck: These crates are used for property-based testing, which is a different but complementary technique to fuzzing. Property-based testing generates many random inputs to verify properties of your code.

In almost all cases, cargo fuzz will be your primary tool for fuzzing Rust code. It uses libFuzzer, a powerful and modern fuzzing engine, and simplifies the fuzzing process significantly.  Make sure you define good fuzz targets and use sanitizers to detect errors effectively.

</div>
