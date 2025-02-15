# Fuzzing

{{#include fuzzing.incl.md}}

## `afl` {#afl}

[![afl][c-afl-badge]][c-afl] [![afl-crates.io][c-afl-crates.io-badge]][c-afl-crates.io] [![afl-github][c-afl-github-badge]][c-afl-github] [![afl-lib.rs][c-afl-lib.rs-badge]][c-afl-lib.rs]{{hi:afl}}

Fuzz testing (or fuzzing) is a software testing technique used to find security and stability issues by providing pseudo-random data as input to the software. AFLplusplus is a popular, effective, and modern fuzz testing tool based on AFL. `afl.rs` allows one to run AFLplusplus on code written in the Rust programming language.

```rust,editable
{{#include ../../../crates/cats/development_tools_testing/tests/afl.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[fuzzing: review fuzzing crates (P1)](https://github.com/john-cd/rust_howto/issues/339)

Fuzzing (for example, with cargo-fuzz) is a unguided approach to random testing. A fuzzer generally provides an input of random bytes, and then examines fairly generic properties (such as "doesn't crash" or "commit undefined behavior") about the resulting program.

Fuzzers generally get their power through a kind of evolutionary algorithm that rewards new mutant inputs that "discover" new branches of the program under test. Fuzzers are excellent for testing security boundaries, precisely because they make no validity assumptions (hence, they are "unguided") when generating the input.

TODO P2 cover [cargo-fuzz](https://github.com/rust-fuzz/cargo-fuzz)

TODO P2 cover property testing [proptest](https://github.com/proptest-rs/proptest)

```toml
[dev-dependencies]
proptest = "1.0.0"
```

TODO P2 cover [bolero](https://github.com/camshaft/bolero/)

`bolero`

TODO P2 cover [valgrind](https://valgrind.org/)

`valgrind`

</div>
