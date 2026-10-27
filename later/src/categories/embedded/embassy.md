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

Mark async functions with `#[embassy_executor::task]` and spawn them with the executor's `Spawner`. `embassy-time` provides `Timer` and periodic `Ticker` types, while bounded `embassy-sync` channels pass data between tasks. A time driver must be selected at build time, usually by the HAL. The registered `embedded` example selects Embassy's `platform-std` executor and standard-library time driver so Cargo can exercise producer and consumer tasks on the host. That host setup demonstrates task communication; it is not firmware configuration.

The task module and firmware entry point are separate files. Place both in a firmware crate configured for the chosen chip and executor.

```rust,editable
{{#include ../../../crates/cats/embedded/examples/embassy/embassy.rs:example}}
```

```rust,editable
{{#include ../../../crates/cats/embedded/examples/embassy/main.rs:example}}
```

### RP2040 blinky firmware

The following firmware sketch uses the RP2040 HAL and is intentionally not compiled by the host example workspace. It toggles the onboard LED every 500 milliseconds:

```rust,ignore
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_25, Level::Low);

    loop {
        led.toggle();
        Timer::after_millis(500).await;
    }
}
```

For a Pico with a compatible debug probe, add a runner to the firmware project's `.cargo/config.toml`:

```toml
[target.thumbv6m-none-eabi]
runner = "probe-rs run --chip RP2040"
```

Install the target and `probe-rs`, then build and flash with Cargo:

```powershell
rustup target add thumbv6m-none-eabi
cargo install probe-rs-tools --locked
cargo run --release --target thumbv6m-none-eabi
```

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
