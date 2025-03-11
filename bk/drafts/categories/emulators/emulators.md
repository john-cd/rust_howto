# Emulators

{{#include emulators.incl.md}}

## Emulators {#emulators}

```rust,editable
{{#include ../../../crates/cats/emulators/tests/emulator.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[emulators: write](https://github.com/john-cd/rust_howto/issues/347)

## Virtual Machines / CPU Emulators {#skip}

Mention `polkavm` (used in smart contracts, etc.) - ~500k downloads.
PolkaVM is a general purpose user-level RISC-V based virtual machine. This project is still unfinished and is a very heavy work-in-progress.

https://medium.com/@OneBlockplus/in-depth-analysis-of-polkavm-a-perfect-path-to-understanding-polkadot-2-0-6ac347a296ba

## ROM loading {#skip}

`ihex` is a Rust library for [parsing][p-parsing] and generating Intel HEX (or IHEX) objects. This format is commonly used for representing compiled program code and data to be loaded into a microcontroller, flash memory or ROM.

</div>
