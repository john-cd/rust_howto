<!-- TODO Reviewed ??-??-2026. -->

# Code Verification and Formal Methods

{{#include code_verification.incl.md}}

[Rust formal methods][rust-formal-methods~website]↗{{hi:Formal methods}}.

_Formal methods_ use mathematical models and logic to state and check properties of software and hardware. They are useful when selected correctness or safety properties need stronger evidence than testing alone.

_Program verification_ is the process of formally proving the correctness of a program. This involves analyzing the code and ensuring that it meets specific properties, such as:

- Memory safety: The program does not have memory leaks, buffer overflows, or other memory-related errors.
- Thread safety: The program can be executed concurrently without causing data races or other [concurrency][p~concurrency] issues.
- Functional correctness: The program produces the correct output for all valid inputs.
- [Performance](../../development-tools_profiling/index.md): The program meets specific performance requirements, such as execution time or memory usage.

Rust's type system and compiler reject many invalid programs, but passing compilation does not prove that a program meets every behavioral or safety property. Choose verification methods based on the property you need to check:

- **Static checks** inspect code without running a particular input. The compiler and [`clippy`][c~clippy~docs]↗{{hi:clippy}} catch classes of type, safety, and quality problems.
- **Execution-based checks** run selected inputs or schedules. Unit and property tests, fuzzing, and [`miri`][ex~other~miri]↗{{hi:miri}} can expose bugs on the paths they exercise. Miri interprets Rust programs and detects certain forms of undefined behavior; it does not prove that unexecuted paths are correct.
- **Formal verification** uses a model and stated properties to reason about a program. [`kani`][c~kani~docs]↗{{hi:kani}} model-checks proof harnesses over bounded inputs and can verify assertions and some safety properties. A proof applies to the modeled harness and supported operations, not automatically to every property of the whole application.

## Verify Rust Properties with `kani` {#code-verifiers}

{{#badges kani}} {{hi:Model-checking}} {{hi:Verification}} {{#categories development-tools="Development tools"}} {{hi:kani}}

[`kani`][c~kani~repo]↗ is a model checker for Rust. A proof harness states a property and lets Kani explore the harness's bounded inputs and supported operations. It can expose counterexamples for assertions, arithmetic overflow, and some unsafe-code behavior. A proof only applies to the harness, bounds, and operations that Kani analyzes; it does not establish that the entire application is correct.

Install the verifier and its supporting tools, then run the proof harnesses in the package or workspace:

```sh
cargo install --locked kani-verifier
cargo kani setup
cargo kani
```

Consult the [Kani documentation][c~kani~docs]↗ for current host requirements and supported Rust features. Kani's build differs from ordinary tests: it analyzes `#[kani::proof]` harnesses rather than executing `#[test]` functions. Use concurrency-specific tools such as Loom or Shuttle when the property depends on exploring thread schedules.

```rust,editable
{{#include ../../../../crates/cats/development_tools/src/kani.rs:example}}
```

## Prove a Function Contract with Creusot {#creusot-contracts}

[`Creusot`][creusot~repo]↗ is a deductive verifier that checks whether Rust code satisfies stated preconditions and postconditions. It complements tests and Miri, which execute selected inputs, and bounded model checking with Kani, which explores a harness's bounded inputs: Creusot attempts to prove the contract for every input that satisfies its precondition. The result applies only to the stated contract and code that Creusot supports; an incomplete contract does not establish a broader behavior.

The verifier uses Rust compiler internals and requires the exact nightly specified by [Creusot 0.14.0](https://github.com/creusot-rs/creusot/blob/v0.14.0/rust-toolchain). Update the compiler pin alongside a compatible Creusot release.

This independent example pins Creusot 0.14.0, `creusot-std` 0.14.0, and the required Rust toolchain `nightly-2026-08-03`. On a supported Linux or macOS host with Cargo, Opam, `curl`, and the [Creusot build prerequisites][creusot~guide~v0.14.0], initialize Opam and install the tagged release:

```sh
opam init --disable-sandboxing --bare --yes
git clone --depth 1 --branch v0.14.0 https://github.com/creusot-rs/creusot.git
cd creusot
./INSTALL
```

From `bk/crates/cats/development_tools/examples/creusot_contract`, run the accepted proof:

```sh
cargo creusot -- --locked
```

The precondition excludes exactly `i64::MAX`, so `x + 1` cannot overflow. The postcondition compares mathematical integer views of the input and result. The feature-gated version deliberately returns `x`; enabling it adds a failing postcondition obligation:

```sh
cargo creusot -- --locked --features deliberately-fails
```

The deliberately failing command returns a nonzero status because the postcondition for `add_one_wrong` is unproved. This proof covers only `add_one` for inputs satisfying its precondition. It complements, rather than replaces, tests, Miri, and Kani's bounded harnesses. The [Verify Rust Standard Library effort][verify-rust-std~guide]↗ provides upstream context for applying verification to Rust code.

```rust,editable
{{#include ../../../../crates/cats/development_tools/examples/creusot_contract/src/lib.rs:example}}
```

## Concurrent Code Verification {#concurrent-code-verification}

Concurrency testing tools explore schedules through a model of the program. [`loom`][c~loom~docs]↗{{hi:loom}} systematically explores executions represented by its synchronization primitives, while [`shuttle`][c~shuttle~docs]↗{{hi:shuttle}} runs concurrent tests under a controlled scheduler and can explore many schedules. Both require the test to use the tool's modeled APIs; neither proves arbitrary concurrent code correct.

### `shuttle` {#shuttle}

{{#badges shuttle development-tools::testing=Testing asynchronous=Asynchronous concurrency=Concurrency}}{{hi:Async}}{{hi:Concurrency}}{{hi:Lock}}{{hi:Thread}}

[`shuttle`][c~shuttle~docs]↗{{hi:shuttle}} is a library for testing concurrent Rust code.

### `loom` {#loom}

{{#badges loom concurrency=Concurrency data-structures="Data structures"}}{{hi:Lock-free}}{{hi:Atomic}}

[`loom`][c~loom~docs]↗{{hi:loom}} allows permutation testing for concurrent code.

## Other Tools {#other-tools .skip}

- [MIRAI][mirai~repo]↗.
- [`prusti`][prusti~website]↗ is an automated program verifier for Rust, based on the Viper infrastructure. It leverages Rust's strong type guarantees to simplify the specification and verification of Rust programs.
- [Creusot][creusot~repo]↗ helps prove that code is correct in an automated fashion.
- [`crucible`][c~crucible~docs]↗{{hi:crucible}} (symbolic execution).

[Creusot's manual][creusot~guide]↗ covers writing contracts and proving their obligations. The [Verify Rust Standard Library effort][verify-rust-std~guide]↗ tracks verification targets and harnesses for standard-library code.

## References {#references .skip}

- [Model Checking][wikipedia~model-checking]↗.
- [Kani Rust Verifier][c~kani~website]↗.
- [Announcing the Kani Rust Verifier Project][c~kani~announcing-the-kani-rust-verifier-project~blog]↗.
- [Using the Kani Rust Verifier on a Firecracker Example][c~kani~using-the-kani-rust-verifier-on-a-firecracker~example~blog]↗.
- [Using the Kani Rust Verifier on a Rust Standard Library CVE][c~kani~using-the-kani-rust-verifier-on-a-rust-standard-library-cve]↗.

## Related Topics {#related-topics .skip}

- [Testing][p~testing], including property-based and [fuzz testing][p~fuzzing].
- [Concurrency][p~concurrency] and model-based schedule testing.
- Static analysis and [code formatting](../formatting/formatting.md), including compiler and `clippy` checks.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
