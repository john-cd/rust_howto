# Code Verification and Formal Methods

[Rust formal methods][rust-formal-methods-website]{{hi:Formal methods}}⮳

Formal Methods are a collection of mathematically rigorous techniques used to specify, design, and verify software and hardware systems. These methods employ formal logic and mathematical proofs to ensure the correctness and reliability of systems, especially those that are safety-critical or security-critical.

Program verification is the process of formally proving the correctness of a program. This involves analyzing the code and ensuring that it meets specific properties, such as:

- Memory safety: The program does not have memory leaks, buffer overflows, or other memory-related errors.
- Thread safety: The program can be executed concurrently without causing data races or other concurrency issues.
- Functional correctness: The program produces the correct output for all valid inputs.
- Performance: The program meets specific performance requirements, such as execution time or memory usage.

There are two main approaches to Rust program verification:

1. Static verification: This involves analyzing the code at compile time to identify potential errors and prove the correctness of certain properties. Rust's type system and ownership model already provide a strong foundation for static verification. Additionally, tools like `miri` and `kani` can be used to perform more advanced static analysis.

2. Dynamic verification: This involves running the program with different inputs and checking its behavior against expected results. Techniques like fuzz testing can be used to identify potential issues.

## Verify Rust code {#code-verifiers}

[![kani][c-kani-badge]][c-kani]{{hi:kani}}
[![kani-crates.io][c-kani-crates.io-badge]][c-kani-crates.io]
[![kani-github][c-kani-github-badge]][c-kani-github]
[![kani-lib.rs][c-kani-lib.rs-badge]][c-kani-lib.rs]

[kani][c-kani-github]{{hi:kani}}⮳ is a Rust verifier.

```rust,editable
{{#include ../../../../deps/tests/categories/development_tools/other/kani.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
TODO P1 expand; revise refs.incl.md
</div>
