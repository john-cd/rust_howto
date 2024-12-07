# Others

{{#include other.incl.md}}

## Check your Rust code in the background {#bacon}

[![bacon][c-bacon-badge]][c-bacon]{{hi:bacon}}
[![bacon-crates.io][c-bacon-crates.io-badge]][c-bacon-crates.io]
[![bacon-github][c-bacon-github-badge]][c-bacon-github]
[![bacon-lib.rs][c-bacon-lib.rs-badge]][c-bacon-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}

`bacon` is a background rust code checker. It is designed for minimal interaction, so that you can just let it run, alongside your editor, and be notified of warnings, errors, or test failures in your Rust code.

```sh
# Install or update `bacon`
cargo install --locked bacon

# Check the current project
bacon

# Run clippy instead of cargo check
bacon clippy
```

## Search for APIs {#roogle}

[`Roogle`][c-roogle-website]{{hi:roogle}}⮳ [![roogle-github][c-roogle-github-badge]][c-roogle-github] [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

Roogle is a Rust API search engine, which allows you to search functions by names and type signatures.

The query cab

|||
|---|---|
| fn f(type) -> type |  |
| fn (type) -> type | fn (&char) -> bool |
| fn(type) -> type |  |
| (type) -> type| (&mut Vec<T>, value: T) |

## Deployment {#shuttle-rs}

[shuttle.rs][shuttle-rs-website]{{hi:shuttle.rs}}⮳ [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

```rust,editable
{{#include ../../../../deps/tests/categories/development_tools/other/shuttle.rs:example}}
```

## Binary minimizer {#binary-minimizer}

[How to minimize Rust binary size][min-sized-rust-github]⮳

By default, Rust optimizes for execution speed, compilation speed, and ease of debugging. This approach is suitable for most applications, as it balances performance and developer productivity. However, in specific scenarios where binary size is a critical concern (e.g., embedded systems or deployment to constrained environments), Rust offers mechanisms to optimize for smaller binary sizes.

```sh
cargo build --release
```

## Code generators {#code-generators}

[Top artificial intelligence tools that can generate code to help programmers][blog-ai-tools-that-can-generate-code]⮳

## Code verifier {#code-verifiers}

[![kani][c-kani-badge]][c-kani]{{hi:kani}}
[![kani-crates.io][c-kani-crates.io-badge]][c-kani-crates.io]
[![kani-github][c-kani-github-badge]][c-kani-github]
[![kani-lib.rs][c-kani-lib.rs-badge]][c-kani-lib.rs]

[kani][c-kani-github]{{hi:kani}}⮳ is a Rust verifier.

```rust,editable
{{#include ../../../../deps/tests/categories/development_tools/other/kani.rs:example}}
```

## Formal Methods {#formal-methods}

[Rust formal methods][rust-formal-methods-website]{{hi:Formal methods}}⮳

Formal Methods are a collection of mathematically rigorous techniques used to specify, design, and verify software and hardware systems. These methods employ formal logic and mathematical proofs to ensure the correctness and reliability of systems, especially those that are safety-critical or security-critical.

Rust program verification is the process of formally proving the correctness of a Rust program. This involves using mathematical techniques and tools to analyze the code and ensure that it meets specific properties, such as:

- Memory safety: The program does not have memory leaks, buffer overflows, or other memory-related errors.
- Thread safety: The program can be executed concurrently without causing data races or other concurrency issues.
- Functional correctness: The program produces the correct output for all valid inputs.
- Performance: The program meets specific performance requirements, such as execution time or memory usage.

There are two main approaches to Rust program verification:

1. Static verification: This involves analyzing the code at compile time to identify potential errors and prove the correctness of certain properties. Rust's type system and ownership model already provide a strong foundation for static verification. Additionally, tools like `miri` and `kani` can be used to perform more advanced static analysis.

2. Dynamic verification: This involves running the program with different inputs and checking its behavior against expected results. Techniques like fuzz testing can be used to identify potential issues.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
TODO P1 expand; revise refs.incl.md
</div>
