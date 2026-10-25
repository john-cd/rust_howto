<!-- TODO Reviewed ??-??-2026 -->

# Embassy and Friends

{{#include embassy.incl.md}}

## Embassy {#embassy}

[![Embassy][embassy~repo~badge]][embassy~repo]

[![embassy][c~embassy~docs~badge]][c~embassy~docs]{{hi:embassy}}
[![embassy~crates.io][c~embassy~crates.io~badge]][c~embassy~crates.io]
[![embassy~repo][c~embassy~repo~badge]][c~embassy~repo]
[![embassy~lib.rs][c~embassy~lib.rs~badge]][c~embassy~lib.rs]

Embassy is a set of libraries for asynchronous embedded applications. Its executor schedules statically allocated tasks without requiring a heap. A firmware project still needs exactly one executor platform, normally supplied by its chip-specific HAL.

### Timers and tasks

Mark async functions with `#[embassy_executor::task]` and spawn them with the executor's `Spawner`. `embassy-time` provides `Timer` and periodic `Ticker` types, while bounded `embassy-sync` channels pass data between tasks. A time driver must be selected at build time, usually by the HAL.

The task module and firmware entry point are separate files. Place both in a firmware crate configured for the chosen chip and executor.

```rust,editable
{{#include ../../../crates/cats/embedded/examples/embassy/embassy.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/embedded/examples/embassy/main.rs:example}}
```

### Synchronization and chip support

Choose the HAL for the actual device: [`embassy-stm32`][c~embassy-stm32~docs], [`embassy-nrf`][c~embassy-nrf~docs], and [`embassy-rp`][c~embassy-rp~docs] expose chip-specific peripheral setup and executor support. Check each HAL's feature list and examples because configuration differs by chip.

Firmware examples need a target triple, linker configuration, and a matching HAL/time driver. Run them on the board or a supported emulator; they are not part of the host example workspace. Use [probe-rs][ex~embedded~flash~probe-rs] to flash and debug a supported board.

## Related Topics {#related-topics .skip}

- [[async | Asynchronous programming]].
- [[hals | Embedded hardware abstraction layers]].
- [[rtos | Real-time operating systems and low-power design]].
- [[pacs | Peripheral access crates]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
