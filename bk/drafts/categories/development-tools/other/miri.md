# Miri Interpreter

{{#include miri.incl.md}}

## Detect Undefined Behavior with the `miri` Interpreter {#miri}

[![miri~github][c~miri~github~badge]][c~miri~github]{{hi:miri}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`miri`][c~miri~github]⮳ is an experimental interpreter{{hi:Rust interpreter}} for Rust's mid-level intermediate representation{{hi:Intermediate representation}} (MIR{{hi:MIR}}). It can run binaries and test suites of [`cargo`][c~cargo~docs]⮳{{hi:cargo}} projects and detect certain classes of undefined behavior. It can run binaries and test suites of [`cargo`][c~cargo~docs]⮳{{hi:cargo}} projects and detect unsafe code that fails to uphold its safety requirements. It can also perform cross-interpretation{{hi:Cross-interpretation}} for arbitrary foreign targets.

## Install the `miri` Interpreter {#miri-installation}

[`miri`][c~miri~docs]⮳{{hi:miri}}

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
[miri: polish](https://github.com/john-cd/rust_howto/issues/304)
</div>
