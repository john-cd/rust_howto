# Fuzzing

{{#include fuzzing.incl.md}}

## `afl` {#afl}

[![afl][c-afl-badge]][c-afl] [![afl-crates.io][c-afl-crates.io-badge]][c-afl-crates.io] [![afl-github][c-afl-github-badge]][c-afl-github] [![afl-lib.rs][c-afl-lib.rs-badge]][c-afl-lib.rs]{{hi:afl}}

Fuzz testing is a software testing technique used to find security and stability issues by providing pseudo-random data as input to the software. AFLplusplus is a popular, effective, and modern fuzz testing tool based on AFL. This library, afl.rs, allows one to run AFLplusplus on code written in the Rust programming language.

```rust,editable
{{#include ../../../deps/tests/categories/development_tools_testing/afl.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 review fuzzing crates
</div>
