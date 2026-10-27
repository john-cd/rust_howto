<!-- TODO Reviewed ??-??-2026 -->

# Emulators

{{#include emulators.incl.md}}

## Emulators {#emulators}

```rust,editable
{{#include ../../../crates/cats/emulators/examples/emulators/emulator.rs:example}}
```

## RISC-V Virtual Machines {#virtual-machines-cpu-emulators}

[`PolkaVM`][polkavm~repo]{{hi:polkavm}} is a general-purpose, user-level virtual machine based on RISC-V. Its maintainers list sandboxing among its design goals and describe the project as unfinished, so check the upstream documentation for its current status before choosing it.

## ROM Loading {#rom-loading}

Intel HEX (IHEX) is a text format for program data used by microcontrollers, flash memory, and ROM. The `ihex` reader parses records with checksums and exposes data offsets and byte values:

```rust,editable
{{#include ../../../crates/cats/emulators/examples/emulators/intel_hex.rs:example}}
```

See the [`ihex` documentation][c~ihex~docs] and the [parsing chapter][p~parsing] for more formats and parser design.

## Related Topics {#related-topics .skip}

- [[virtualization | Virtualization]] and [[embedded | Embedded Systems]] for related guest and firmware topics.
- [[graphics | Graphics]] and [[audio | Audio]] for display and sound output.
- [[development-tools_debugging | Development Tools: Debugging]] for diagnostics.
- [[parsing | Parsing]] for additional binary formats.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [emulators: write](https://github.com/john-cd/rust_howto/issues/347)
  [in-depth-analysis-of-polkavm-a-perfect-path-to-understanding-polkadot][blog~medium~in-depth-analysis-of-polkavm].
</div>
