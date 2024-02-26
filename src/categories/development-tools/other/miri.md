# Miri Interpreter

[![miri-github][miri-github-badge]][miri-github]  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]

[Miri][miri-github]⮳ is an experimental interpreter for Rust's mid-level intermediate representation (MIR). It can run binaries and test suites of cargo projects and detect certain classes of undefined behavior. It can also perform cross-interpretation for arbitrary foreign targets.

## Installation

```bash
rustup +nightly component add miri
cargo clean
cargo miri test
# or
cargo miri run
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}
