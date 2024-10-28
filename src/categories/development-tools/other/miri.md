# Miri Interpreter

[![miri-github][c-miri-github-badge]][c-miri-github]{{hi:miri}}  [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}

[Miri][c-miri-github]⮳ is an experimental interpreter{{hi:Interpreters}} for Rust's mid-level intermediate representation{{hi:Intermediate representation}} (MIR{{hi:MIR}}). It can run binaries and test suites of cargo projects and detect certain classes of undefined behavior. It can also perform cross-interpretation{{hi:Cross-interpretation}} for arbitrary foreign targets.

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

<div class="hidden">
</div>
