# Code Verification and Formal Methods

{{#include code_verification.incl.md}}

[Rust formal methods][rust-formal-methods-website]{{hi:Formal methods}}⮳

Formal Methods are a collection of mathematically rigorous techniques used to specify, design, and verify software and hardware systems. These methods employ formal logic and mathematical proofs to ensure the correctness and reliability of systems, especially those that are safety-critical or security-critical.

Program verification is the process of formally proving the correctness of a program. This involves analyzing the code and ensuring that it meets specific properties, such as:

- Memory safety: The program does not have memory leaks, buffer overflows, or other memory-related errors.
- Thread safety: The program can be executed concurrently without causing data races or other [concurrency][p-concurrency] issues.
- Functional correctness: The program produces the correct output for all valid inputs.
- [Performance][p-performance]: The program meets specific [performance][p-performance] requirements, such as execution time or memory usage.

There are two [main][p-main] approaches to Rust program verification:

1. Static verification: This involves analyzing the code at compile time to identify potential errors and prove the correctness of certain properties. Rust's type system and ownership model already provide a strong foundation for static verification. Additionally, tools like [`miri`][c-miri]⮳{{hi:miri}} and [`kani`][c-kani]⮳{{hi:kani}} can be used to perform more advanced static analysis.

2. Dynamic verification: This involves running the program with different inputs and checking its behavior against expected results. Techniques like fuzz [testing][p-testing] can be used to identify potential issues.

## Verify your Rust code {#code-verifiers}

[![kani][c-kani-badge]][c-kani] [![kani-crates.io][c-kani-crates.io-badge]][c-kani-crates.io] [![kani-github][c-kani-github-badge]][c-kani-github] [![kani-lib.rs][c-kani-lib.rs-badge]][c-kani-lib.rs]{{hi:kani}}{{hi:Model-checking}}{{hi:Verification}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}{{hi:kani}}

[kani][c-kani-github]⮳ is a Rust verifier / model checker. A model checker formally verifies that a system like a software program meets a given specification, using mathematical techniques (automated reasoning) to prove that a system satisfies a property for all possible states and behaviors. Model checkers explore the entire state space of the system, as opposed to approaches like [fuzzing][p-fuzzing] and property [testing][p-testing].

Model checking is a valuable technique for verifying the correctness of critical systems, such as safety-critical software and communication protocols. Kani is particularly useful for verifying unsafe code blocks. Some example properties you can prove with Kani include memory safety (e.g., null pointer dereferences, use-after-free, etc.), the absence of certain runtime errors (i.e., index out of bounds, panics), the absence of some types of unexpected behavior (e.g., arithmetic overflows), in addition to user-specified assertions.

At present, Kani does not support verifying concurrent code.

Kani offers an easy installation [option][p-option] on three platforms:

- x86_64-unknown-linux-gnu (Most Linux distributions)
- x86_64-apple-darwin (Intel Mac OS)
- aarch64-apple-darwin (Apple Silicon Mac OS)

Python version 3.7 or newer and the package installer `pip` must be installed.

Install with:

```sh
cargo install --locked kani-verifier
```

Then [download][p-download] the Kani compiler and other necessary dependencies, and place them under ~/.kani/ by default:

```sh
cargo kani setup
```

Run kani:

```sh
cargo kani [OPTIONS]
```

Kani works like `cargo test` except that it will analyze "proof harnesses" instead of running test harnesses.

```rust,editable
{{#include ../../../../crates/cats/development_tools/src/kani.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[code_verification: expand; revise refs.incl.md (P1)](https://github.com/john-cd/rust_howto/issues/303)

Link to:

Static Analysis/Linting: clippy (for catching common code errors and style issues)
Formal Verification: [`kani`][c-kani]⮳{{hi:kani}} (model checker), crucible (symbolic execution) - These are more advanced and complex tools.
Property-Based [Testing][p-testing]: proptest, quickcheck
Unit Testing: std::test (built-in [testing][p-testing] framework)
Integration Testing: (Often uses std::test but focuses on [testing][p-testing] interactions between modules or components)
Fuzzing: [cargo][p-cargo] fuzz, afl.rs (bindings to AFL)
Type Checking: (Built into the Rust compiler)
Code Review Tools: (Not Rust-specific, but used in conjunction with Rust code. Examples: GitHub, GitLab, etc.)

TODO P1 add [Model_checking](https://en.wikipedia.org/wiki/Model_checking) [getting-started](https://model-checking.github.io/kani/getting-started.html)
[announcing-the-kani-rust-verifier-project](https://model-checking.github.io/kani-verifier-blog/2022/05/04/announcing-the-kani-rust-verifier-project.html)
[using-the-kani-rust-verifier-on-a-firecracker-example](https://model-checking.github.io/kani-verifier-blog/2022/07/13/using-the-kani-rust-verifier-on-a-firecracker-example.html)
[sing-the-kani-rust-verifier-on-a-rust-standard-library-cve](https://model-checking.github.io/kani-verifier-blog/2022/06/01/using-the-kani-rust-verifier-on-a-rust-standard-library-cve.html)

TODO P2 cover [loom](https://github.com/tokio-rs/loom) and [shuttle](https://github.com/awslabs/shuttle) for [concurrency][p-concurrency] testing. Loom attempts to check all possible interleavings, while Shuttle chooses interleavings randomly. The former is sound (like Kani), but the latter is more scalable to large problem spaces (like property [testing][p-testing]).

TODO P2 cover [MIRAI](https://github.com/endorlabs/MIRAI)

TODO P2 cover [prusti](https://www.pm.inf.ethz.ch/research/prusti.html) [creusot](https://github.com/creusot-rs/creusot)
</div>
