<!-- TODO Reviewed ??-??-2026. -->

# Embassy and Friends

{{#include embassy.incl.md}}

## Embassy {#embassy}

[![Embassy][embassy~repo~badge]][embassy~repo]

{{#badges embassy}}

Embassy is a set of libraries for asynchronous embedded applications. Its executor schedules statically allocated tasks without requiring a heap. A firmware project still needs exactly one executor platform, normally supplied by its chip-specific HAL.

### Timers and tasks

Mark async functions with `#[embassy_executor::task]` and spawn them with the executor's `Spawner`. `embassy-time` provides `Timer` and periodic `Ticker` types, while bounded `embassy-sync` channels pass data between tasks. A time driver must be selected at build time, usually by the HAL. The registered `embedded` example selects Embassy's `platform-std` executor and standard-library time driver so Cargo can exercise producer and consumer tasks on the host. That host setup demonstrates task communication; it is not firmware configuration.

Run the host example from `bk/platform/cats/embedded` with `cargo run --locked --example embassy`. The bounded channel transfers readings between the tasks. A deadline keeps the check finite if a task stops sending.

```rust,noplayground
{{#include ../../../platform/cats/embedded/examples/embassy/embassy.rs:example}}
```

### RP2040 blinky firmware

The standalone firmware below targets the **original Raspberry Pi Pico (RP2040)**. Its onboard LED is wired to GPIO 25. The Pico W routes its onboard LED through the wireless chip and needs a different driver. The hardware timer wakes the task every 500 milliseconds without a busy loop.

```rust,noplayground
{{#include ../../../platform/cats/embedded/rp2040_blinky/src/main.rs:example}}
```

Install the Cortex-M0+ target and link the standalone project from the repository root:

```sh
rustup target add thumbv6m-none-eabi
just cargo build --locked --release --target thumbv6m-none-eabi --manifest-path bk/platform/cats/embedded/rp2040_blinky/Cargo.toml
```

The project includes the RP2040 boot-stage linker script and a memory map for the Pico's 2 MiB flash and 264 KiB SRAM. The build produces an ARM ELF. The firmware enables the HAL's RP2040 thread executor and time driver while the host example uses its own standard-library configuration.

For a physical check, connect an SWD probe and power the board according to the probe's wiring guide. With `probe-rs` installed, run `probe-rs run --chip RP2040 /path/to/rp2040_blinky`, using the ELF from Cargo's `thumbv6m-none-eabi/release` directory. Look for the LED to alternate on and off every half second. If programming succeeds and the LED stays dark, confirm the board model and LED pin before changing the timer or executor configuration.

### Synchronization and chip support

Choose the HAL for the actual device: [`embassy-stm32`][c~embassy-stm32~docs], [`embassy-nrf`][c~embassy-nrf~docs], and [`embassy-rp`][c~embassy-rp~docs] expose chip-specific peripheral setup and executor support. Check each HAL's feature list and examples because configuration differs by chip.

Other firmware examples need a target triple, linker configuration, and a matching HAL/time driver. Run them on the board or a supported emulator. Use [probe-rs][ex~embedded~flash~probe-rs] to flash and debug a supported board.

## Related Topics {#related-topics .skip}

- [[async | Asynchronous programming]].
- [[hals | Embedded hardware abstraction layers]].
- [[rtos | Real-time operating systems and low-power design]].
- [[pacs | Peripheral access crates]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
