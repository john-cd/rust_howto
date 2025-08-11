# Code Verification and Formal Methods

{{#include code_verification.incl.md}}

[Rust formal methods][rust-formal-methods~website]{{hi:Formal methods}}↗.

_Formal methods_ are a collection of mathematically rigorous techniques used to specify, design, and verify software and hardware systems. These methods employ formal logic and mathematical proofs to ensure the correctness and reliability of systems, especially those that are safety-critical or security-critical.

_Program verification_ is the process of formally proving the correctness of a program. This involves analyzing the code and ensuring that it meets specific properties, such as:

- Memory safety: The program does not have memory leaks, buffer overflows, or other memory-related errors.
- Thread safety: The program can be executed concurrently without causing data races or other [concurrency][p~concurrency] issues.
- Functional correctness: The program produces the correct output for all valid inputs.
- [Performance][p~performance]: The program meets specific [performance][p~performance] requirements, such as execution time or memory usage.

There are two main approaches to Rust program verification:

1. Static verification: This involves analyzing the code at compile time to identify potential errors and prove the correctness of certain properties. Rust's type system and ownership model already provide a strong foundation for static verification. Additionally, tools like [`miri`][miri]↗{{hi:miri}} and [`kani`][c~kani~docs]↗{{hi:kani}} can be used to perform more advanced static analysis.

2. Dynamic verification: This involves running the program with different inputs and checking its behavior against expected results. Techniques like fuzz [testing][p~testing] can be used to identify potential issues.

## Verify Your Rust Code with `kani` {#code-verifiers}

[![kani][c~kani~docs~badge]][c~kani~docs] [![kani~crates.io][c~kani~crates.io~badge]][c~kani~crates.io] [![kani~github][c~kani~github~badge]][c~kani~github] [![kani~lib.rs][c~kani~lib.rs~badge]][c~kani~lib.rs]{{hi:kani}}{{hi:Model-checking}}{{hi:Verification}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}{{hi:kani}}

[kani][c~kani~github]↗ is a Rust verifier / model checker. A model checker formally verifies that a system like a software program meets a given specification, using mathematical techniques (automated reasoning) to prove that a system satisfies a property for all possible states and behaviors. Model checkers explore the entire state space of the system, as opposed to approaches like [fuzzing][p~fuzzing] and property [testing][p~testing].

Model checking is a valuable technique for verifying the correctness of critical systems, such as safety-critical software and communication protocols. Kani is particularly useful for verifying unsafe code blocks. Some example properties you can prove with Kani include memory safety (e.g., null pointer dereferences, use-after-free, etc.), the absence of certain runtime errors (i.e., index out of bounds, panics), the absence of some types of unexpected behavior (e.g., arithmetic overflows), in addition to user-specified assertions.

At present, Kani does not support verifying concurrent code.

Kani offers an easy installation [option][p~option] on three platforms:

- x86_64-unknown-linux-gnu (Most Linux distributions).
- x86_64-apple-darwin (Intel Mac OS).
- aarch64-apple-darwin (Apple Silicon Mac OS).

Python version 3.7 or newer and the package installer [`pip`](https://pypi.org/project/pip)↗{{hi:pip}} must be installed.

Install with:

```sh
cargo install --locked kani-verifier
```

Then [download][p~download] the Kani compiler and other necessary dependencies, and place them under ~/.kani/ by default:

```sh
cargo kani setup
```

Run kani:

```sh
cargo kani [OPTIONS]
```

Kani works like [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} except that it will analyze "proof harnesses" instead of running test harnesses.

```rust,editable
{{#include ../../../../crates/cats/development_tools/src/kani.rs:example}}
```

## Concurrent Code Verification {#concurrent-code-verification}

Loom attempts to check all possible interleavings, while Shuttle chooses interleavings randomly. The former is sound (like Kani), but the latter is more scalable to large problem spaces (like property [testing][p~testing]).

### `shuttle` {#shuttle}

[![shuttle][c~shuttle~docs~badge]][c~shuttle~docs] [![shuttle~crates.io][c~shuttle~crates.io~badge]][c~shuttle~crates.io] [![shuttle~github][c~shuttle~github~badge]][c~shuttle~github] [![shuttle~lib.rs][c~shuttle~lib.rs~badge]][c~shuttle~lib.rs]{{hi:shuttle}}{{hi:Async}}{{hi:Concurrency}}{{hi:Lock}}{{hi:Thread}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`shuttle`][c~shuttle~docs]↗{{hi:shuttle}} is a library for testing concurrent Rust code.

### `loom` {#loom}

[![loom][c~loom~docs~badge]][c~loom~docs] [![loom~crates.io][c~loom~crates.io~badge]][c~loom~crates.io] [![loom~github][c~loom~github~badge]][c~loom~github] [![loom~lib.rs][c~loom~lib.rs~badge]][c~loom~lib.rs]{{hi:loom}}{{hi:Lock-free}}{{hi:Atomic}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`loom`][c~loom~docs]↗{{hi:loom}} allows permutation testing for concurrent code.

## Other Tools {#skip}

- [MIRAI][mirai~github]↗.
- [`prusti`][prusti~website]↗ is an automated program verifier for Rust, based on the Viper infrastructure. It leverages Rust's strong type guarantees to simplify the specification and verification of Rust programs.
- [Creusot][creusot~github]↗ helps you prove your code is correct in an automated fashion.
- [`crucible`][c~crucible~docs]↗{{hi:crucible}} (symbolic execution).

## Related Topics {#related-topics}

| Topic | Rust Crates |
|---|---|
| Static Analysis/Linting | [`clippy`][c~clippy~docs]↗{{hi:clippy}} (for catching common code errors and style issues) |
| Property-Based Testing | [`proptest`][c~proptest~docs]↗{{hi:proptest}}, [`quickcheck`][c~quickcheck~docs]↗{{hi:quickcheck}} |
| Unit [Testing][p~testing] | Use [`cargo test`](https://doc.rust-lang.org/cargo/commands/cargo-test.html)↗{{hi:cargo test}} (built-in testing framework) |
| Integration Testing | Often uses `cargo test`. Focuses on testing interactions between modules or components. |
| Fuzzing | [`cargo fuzz`][book~rust-fuzz]↗{{hi:cargo fuzz}}, [`afl.rs`][c~afl~docs]↗{{hi:afl.rs}} (bindings to AFL) |
| Code Review Tools: Not Rust-specific, but used in conjunction with Rust code. Examples: GitHub, GitLab, etc. | |

## References {#references}

- [Model Checking][wikipedia~model-checking]↗.
- [Kani Rust Verifier][c~kani~website]↗.
- [Announcing the Kani Rust Verifier Project][c~kani~announcing-the-kani-rust-verifier-project~blog]↗.
- [Using the Kani Rust Verifier on a Firecracker Example][c~kani~using-the-kani-rust-verifier-on-a-firecracker~example~blog]↗.
- [Using the Kani Rust Verifier on a Rust Standard Library CVE][c~kani~using-the-kani-rust-verifier-on-a-rust-standard-library-cve]↗.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[code_verification: expand; revise refs.incl.md](https://github.com/john-cd/rust_howto/issues/303)
</div>
