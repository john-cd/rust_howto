# Low-level Interaction with the Operating System

{{#include low_level_system_calls.incl.md}}

## Call `libc`, the C Standard Library {#call-libc}

[![libc][c~libc~docs~badge]][c~libc~docs]{{hi:libc}}
[![libc~crates.io][c~libc~crates.io~badge]][c~libc~crates.io]
[![libc~github][c~libc~github~badge]][c~libc~github]
[![libc~lib.rs][c~libc~lib.rs~badge]][c~libc~lib.rs]
[![cat~external-ffi-bindings][cat~external-ffi-bindings~badge]][cat~external-ffi-bindings]{{hi:External FFI bindings}}
[![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}
[![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

Bindings for directly calling [`libc`][c~libc~docs]⮳{{hi:libc}} functions.

```rust,editable
{{#include ../../../crates/cats/os/examples/low_level_system_calls/libc.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[low_level_system_calls: write](https://github.com/john-cd/rust_howto/issues/430)
</div>
