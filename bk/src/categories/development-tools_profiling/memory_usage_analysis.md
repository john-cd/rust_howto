# Memory usage analysis

{{#include memory_usage_analysis.incl.md}}

## Profile heap memory {#dhat}

[![dhat][c-dhat-badge]][c-dhat]{{hi:dhat}}
[![dhat-crates.io][c-dhat-crates.io-badge]][c-dhat-crates.io]
[![dhat-github][c-dhat-github-badge]][c-dhat-github]
[![dhat-lib.rs][c-dhat-lib.rs-badge]][c-dhat-lib.rs]

`dhat` is a library for heap profiling and ad-hoc profiling with DHAT.

```rust,editable
{{#include ../../../crates/cats/development_tools_profiling/src/dhat.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[memory: write (P2)](https://github.com/john-cd/rust_howto/issues/336)
</div>
