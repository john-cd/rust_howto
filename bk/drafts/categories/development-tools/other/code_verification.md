<!-- TODO Reviewed ??-??-2026 -->

# Code Verification and Formal Methods

{{#include code_verification.incl.md}}

[Rust formal methods][rust-formal-methods~website]↗{{hi:Formal methods}}.

_Formal methods_ are a collection of mathematically rigorous techniques used to specify, design, and verify software and hardware systems. These methods employ formal logic and mathematical proofs to ensure the correctness and reliability of systems, especially those that are safety-critical or security-critical.

_Program verification_ is the process of formally proving the correctness of a program. This involves analyzing the code and ensuring that it meets specific properties, such as:

- Memory safety: The program does not have memory leaks, buffer overflows, or other memory-related errors.
- Thread safety: The program can be executed concurrently without causing data races or other [concurrency][p~concurrency] issues.
- Functional correctness: The program produces the correct output for all valid inputs.
- [Performance][p~performance]: The program meets specific [performance][p~performance] requirements, such as execution time or memory usage.

Rust's type system and compiler reject many invalid programs, but passing compilation does not prove that a program meets every behavioral or safety property. Choose verification methods based on the property you need to check:

- **Static checks** inspect code without running a particular input. The compiler and [`clippy`][c~clippy~docs]↗{{hi:clippy}} catch classes of type, safety, and quality problems.
- **Execution-based checks** run selected inputs or schedules. Unit and property tests, fuzzing, and [`miri`][ex~other~miri]↗{{hi:miri}} can expose bugs on the paths they exercise. Miri interprets Rust programs and detects certain forms of undefined behavior; it does not prove that unexecuted paths are correct.
- **Formal verification** uses a model and stated properties to reason about a program. [`kani`][c~kani~docs]↗{{hi:kani}} model-checks proof harnesses over bounded inputs and can verify assertions and some safety properties. A proof applies to the modeled harness and supported operations, not automatically to every property of the whole application.

## Verify Your Rust Code with `kani` {#code-verifiers}

[![kani][c~kani~docs~badge]][c~kani~docs] [![kani~crates.io][c~kani~crates.io~badge]][c~kani~crates.io] [![kani~repo][c~kani~repo~badge]][c~kani~repo] [![kani~lib.rs][c~kani~lib.rs~badge]][c~kani~lib.rs]{{hi:kani}}{{hi:Model-checking}}{{hi:Verification}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}{{hi:kani}}

[kani][c~kani~repo]↗ is a Rust verifier / model checker. A model checker formally verifies that a system like a software program meets a given specification, using mathematical techniques (automated reasoning) to prove that a system satisfies a property for all possible states and behaviors. Model checkers explore the entire state space of the system, as opposed to approaches like [fuzzing][p~fuzzing] and property [testing][p~testing].

Model checking is a valuable technique for verifying the correctness of critical systems, such as safety-critical software and communication protocols. Kani is particularly useful for verifying unsafe code blocks. Some example properties that can be proven with Kani include memory safety (e.g., null pointer dereferences, use-after-free, etc.), the absence of certain runtime errors (i.e., index out of bounds, panics), the absence of some types of unexpected behavior (e.g., arithmetic overflows), in addition to user-specified assertions.

At present, Kani does not support verifying concurrent code.

Kani offers an easy installation [option][p~option] on three platforms:

- x86_64-unknown-linux-gnu (Most Linux distributions).
- x86_64-apple-darwin (Intel Mac OS).
- aarch64-apple-darwin (Apple Silicon Mac OS).

Python version 3.7 or newer and the package installer [`pip`][pypi~pip]↗{{hi:pip}} must be installed.

Install with:

Install the verifier with `cargo install --locked kani-verifier`.

Then [download][p~download] the Kani compiler and other necessary dependencies, and place them under ~/.kani/ by default:

Then install Kani's compiler and dependencies with `cargo kani setup`.

Run kani:

Run it with `cargo kani [OPTIONS]` from the package or workspace containing the proof harnesses. See the [Kani documentation][c~kani~docs]↗ for supported options and platform requirements.

Kani works like [`cargo test`][book~cargo~cargo-test]↗{{hi:cargo test}} except that it will analyze "proof harnesses" instead of running test harnesses.

```rust,editable
{{#include ../../../../crates/cats/development_tools/src/kani.rs:example}}
```

## Concurrent Code Verification {#concurrent-code-verification}

Concurrency testing tools explore schedules through a model of the program. [`loom`][c~loom~docs]↗{{hi:loom}} systematically explores executions represented by its synchronization primitives, while [`shuttle`][c~shuttle~docs]↗{{hi:shuttle}} runs concurrent tests under a controlled scheduler and can explore many schedules. Both require the test to use the tool's modeled APIs; neither proves arbitrary concurrent code correct.

### `shuttle` {#shuttle}

[![shuttle][c~shuttle~docs~badge]][c~shuttle~docs] [![shuttle~crates.io][c~shuttle~crates.io~badge]][c~shuttle~crates.io] [![shuttle~repo][c~shuttle~repo~badge]][c~shuttle~repo] [![shuttle~lib.rs][c~shuttle~lib.rs~badge]][c~shuttle~lib.rs]{{hi:shuttle}}{{hi:Async}}{{hi:Concurrency}}{{hi:Lock}}{{hi:Thread}} [![cat~development-tools::testing][cat~development-tools::testing~badge]][cat~development-tools::testing]{{hi:Testing}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}}

[`shuttle`][c~shuttle~docs]↗{{hi:shuttle}} is a library for testing concurrent Rust code.

### `loom` {#loom}

[![loom][c~loom~docs~badge]][c~loom~docs] [![loom~crates.io][c~loom~crates.io~badge]][c~loom~crates.io] [![loom~repo][c~loom~repo~badge]][c~loom~repo] [![loom~lib.rs][c~loom~lib.rs~badge]][c~loom~lib.rs]{{hi:loom}}{{hi:Lock-free}}{{hi:Atomic}} [![cat~concurrency][cat~concurrency~badge]][cat~concurrency]{{hi:Concurrency}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

[`loom`][c~loom~docs]↗{{hi:loom}} allows permutation testing for concurrent code.

## Other Tools {#other-tools .skip}

- [MIRAI][mirai~repo]↗.
- [`prusti`][prusti~website]↗ is an automated program verifier for Rust, based on the Viper infrastructure. It leverages Rust's strong type guarantees to simplify the specification and verification of Rust programs.
- [Creusot][creusot~repo]↗ helps prove that code is correct in an automated fashion.
- [`crucible`][c~crucible~docs]↗{{hi:crucible}} (symbolic execution).

## References {#references .skip}

- [Model Checking][wikipedia~model-checking]↗.
- [Kani Rust Verifier][c~kani~website]↗.
- [Announcing the Kani Rust Verifier Project][c~kani~announcing-the-kani-rust-verifier-project~blog]↗.
- [Using the Kani Rust Verifier on a Firecracker Example][c~kani~using-the-kani-rust-verifier-on-a-firecracker~example~blog]↗.
- [Using the Kani Rust Verifier on a Rust Standard Library CVE][c~kani~using-the-kani-rust-verifier-on-a-rust-standard-library-cve]↗.

## Related Topics {#related-topics .skip}

- [Testing][p~testing], including property-based and [fuzz testing][p~fuzzing].
- [Concurrency][p~concurrency] and model-based schedule testing.
- Static analysis and [linting][p~formatting] with the compiler and `clippy`.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
