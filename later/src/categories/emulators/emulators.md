<!-- TODO Reviewed ??-??-2026. -->

# Emulators

{{#include emulators.incl.md}}

## Emulators {#emulators}

The two-register VM below fetches an instruction from program memory, decodes its opcode, and executes it. Each instruction occupies two bytes: an opcode and an operand. Opcodes `0x10` and `0x11` load registers, `0x20` adds r1 into r0, `0x30` decrements r1, `0x40` jumps to an instruction index while r1 is nonzero, and `0x00` halts. Add, decrement, and halt require a zero operand; arithmetic wraps at eight bits. Invalid instructions and addresses return errors.

```rust,editable
{{#include ../../../crates/cats/emulators/examples/emulators/register_vm.rs:example}}
```

The hidden host runner loads a six-instruction program that sums 5 down to 1, prints `15`, and limits execution to 64 instructions so a looping guest cannot run forever. Run it from `later/crates` with `cargo run --locked -p emulators --example emulators -- register-vm`.

## RISC-V Virtual Machines {#virtual-machines-cpu-emulators}

[`PolkaVM`][polkavm~repo]{{hi:polkavm}} is a general-purpose, user-level virtual machine based on RISC-V. Its maintainers list sandboxing among its design goals and describe the project as unfinished, so check the upstream documentation for its current status before choosing it.

The host builds a tiny 32-bit guest from two instructions. The guest calls import zero, `host_increment`, then returns to the host. The calling convention passes both the argument and the result in register A0. The callback adds one to the argument and counts how often the guest calls it.

```rust,noplayground
{{#include ../../../crates/cats/emulators/examples/emulators/polkavm_host.rs:example}}
```

From `later/crates`, run:

```sh
cargo run --locked -p emulators --example emulators -- polkavm-host
```

Expect `Guest returned 42; host callback calls: 1.` The recipe generates its guest in memory. To load a local replacement, add its path after `polkavm-host`. The program must export `add_one_with_host` with the same calling convention and use only imports registered by the host.

The interpreter uses synchronous gas metering so a guest that exhausts its budget returns an error. Gas limits guest execution. Account separately for work triggered inside a host callback and keep the capabilities you register small.

## ROM Loading {#rom-loading}

Intel HEX (IHEX) is a text format for program data used by microcontrollers, flash memory, and ROM. The `ihex` reader parses records with checksums and exposes data offsets and byte values:

```rust,editable
{{#include ../../../crates/cats/emulators/examples/emulators/intel_hex.rs:example}}
```

Run `cargo run --locked -p emulators --example emulators -- intel-hex` to print the decoded record. Real loaders must also resolve extended addresses and reject writes outside the emulated memory map.

See the [`ihex` documentation][c~ihex~docs] and the [parsing chapter][p~parsing] for more formats and parser design.

## Related Topics {#related-topics .skip}

- [[virtualization | Virtualization]] and [[embedded | Embedded Systems]] for related guest and firmware topics.
- [[graphics | Graphics]] and [[audio | Audio]] for display and sound output.
- [[development-tools_debugging | Development Tools: Debugging]] for diagnostics.
- [[parsing | Parsing]] for additional binary formats.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [in-depth-analysis-of-polkavm-a-perfect-path-to-understanding-polkadot][blog~medium~in-depth-analysis-of-polkavm].
</div>
