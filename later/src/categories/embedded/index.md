<!-- TODO Reviewed ??-??-2026 -->

# Embedded Systems

[![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded systems}}

Embedded applications interact with device peripherals under limits on memory, energy, and timing. Decide whether firmware uses `no_std`, then select crates that fit the actual target and execution model.

## Requirements

- Choose the board or MCU, Rust target triple, linker script, and chip-specific HAL or PAC before building firmware; for example, install `thumbv7em-none-eabihf` with `rustup target add thumbv7em-none-eabihf`.
- Host tests cover host-compatible logic; firmware examples need the selected target plus physical hardware or a supported emulator such as QEMU, and are not run in ordinary host CI.
- Follow the board and HAL instructions for flashing, debugging, interrupts, and time drivers; [probe-rs][ex~embedded~flash~probe-rs] supports flashing and debugging selected chips.

## Learning Path

1. Inspect a chip's [PAC][ex~embedded~pacs], often generated from an SVD with [`svd2rust`][svd2rust~repo], then use [HALs and `embedded-hal` traits][ex~embedded~embedded-hal] to work with peripherals.
2. Choose startup and concurrency tools for the target: `cortex-m-rt` provides Cortex-M startup, while [RTIC][ex~embedded~rtos~rtic] and [Embassy][ex~embedded~embassy] offer different task models; compare the [RTOS options](rtos.md).
3. Add [sensor drivers][ex~embedded~sensors], [flash storage][ex~embedded~flash~storage], and [probe-based firmware programming][ex~embedded~flash~probe-rs] as needed by the board.
4. Review [embedded logging and support crates][ex~embedded~defmt] for `defmt`, panic handlers such as `panic-halt`, memory, and serialization.

For broader project discovery, browse [Awesome Embedded Rust][awesome-embedded-rust~repo] and [Are We RTOS Yet?][arewertosyet~website]. Check maintenance, target support, and feature compatibility in each project's current documentation before choosing a dependency.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
