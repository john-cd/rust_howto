<!-- TODO Reviewed ??-??-2026 -->
# Embedded Frameworks/HALs (Hardware Abstraction Layers)

{{#include hals.incl.md}}

An embedded hardware abstraction layer (HAL) connects application code to a particular microcontroller or board. A chip-family HAL configures pins, clocks, and peripherals, then implements shared interfaces so device drivers can work across hardware. The [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} crate defines common traits; it is an interface layer, not a complete HAL for a particular chip.

## Choose a HAL for the Exact Target

Start with the full part number and board revision. Two chips in one family can expose different peripherals, pin maps, memory sizes, or errata. Check the HAL's supported device list and board examples before choosing it.

Compare the following details:

- **Peripheral coverage:** confirm that the HAL supports the peripherals and pins your board actually routes.
- **Trait compatibility:** check whether it implements the `embedded-hal` version expected by your device-driver crates. A crate that implements an older trait version may not fit a newer driver without an adapter.
- **Execution model:** choose blocking interfaces for simple foreground work, async traits when the HAL and executor support them, or polling traits when the peripheral is driven that way. The companion crates `embedded-hal-async` and `embedded-hal-nb` provide those alternate interfaces.
- **Board support:** look for working clock, pin, interrupt, and low-power setup examples for your exact board.
- **Maintenance and tooling:** check the supported Rust target, compiler version, release activity, debugging workflow, and documented limitations.

The HAL usually owns the PAC peripherals and pin configuration. Prefer its safe APIs for routine I/O; reach for a PAC directly only when the HAL does not expose a required feature or register.

## Portable Device Drivers

A portable driver is generic over the bus trait it needs. It should not select pins, configure the MCU clock, or assume one vendor's peripheral type. For example, a register read can use the blocking I2C trait:

```rust
use embedded_hal::i2c::I2c;

fn read_register<I2C: I2c>(
    bus: &mut I2C,
    address: u8,
    register: u8,
) -> Result<u8, I2C::Error> {
    let mut value = [0];
    bus.write_read(address, &[register], &mut value)?;
    Ok(value[0])
}
```

The application supplies an I2C implementation from its HAL. A sensor driver can then add the device's register map, units, and error handling while remaining usable with any compatible HAL. For a shared I2C or SPI bus, use the bus-sharing utilities instead of making each driver manage the whole peripheral.

## Key Crates {#key-crates .skip}

- [`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}}: Common traits used by platform-independent drivers.
- `embedded-hal-async`: Async counterparts to selected `embedded-hal` traits.
- `embedded-hal-nb`: Polling counterparts for non-blocking peripheral operations.
- MCU-family HALs: concrete pin, clock, and peripheral APIs for the selected part; examples include `esp-hal` and HALs maintained for STM32 and nRF devices.
- [`cortex-m`][c~cortex-m~docs]↗{{hi:cortex-m}}: Core Cortex-M support such as interrupt and core-peripheral access; it does not replace a chip-specific HAL.

## `embedded-hal` {#embedded-hal}

[![embedded-hal][c~embedded-hal~docs~badge]][c~embedded-hal~docs] [![embedded-hal~crates.io][c~embedded-hal~crates.io~badge]][c~embedded-hal~crates.io] [![embedded-hal~repo][c~embedded-hal~repo~badge]][c~embedded-hal~repo] [![embedded-hal~lib.rs][c~embedded-hal~lib.rs~badge]][c~embedded-hal~lib.rs]{{hi:embedded-hal}}{{hi:Hal}}{{hi:Io}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]{{hi:Hardware support}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`embedded-hal`][c~embedded-hal~docs]↗{{hi:embedded-hal}} is the blocking trait set for portable peripheral drivers. Its I2C, SPI, digital I/O, PWM, and delay traits describe operations; the chip HAL supplies their concrete implementations. Async and polling interfaces live in companion crates.

{{#example embedded-hal}}

The following host-side example implements `embedded-hal`'s output-pin trait for a fake pin. It builds and runs without a board; replace the mock with the pin type provided by the target's HAL in firmware.

{{#include ../../../crates/cats/embedded/examples/hals/main.rs:example}}

## `rppal` {#rppal}

[![rppal][c~rppal~docs~badge]][c~rppal~docs] [![rppal~crates.io][c~rppal~crates.io~badge]][c~rppal~crates.io] [![rppal~repo][c~rppal~repo~badge]][c~rppal~repo] [![rppal~lib.rs][c~rppal~lib.rs~badge]][c~rppal~lib.rs]{{hi:rppal}}{{hi:Embedded-hal}}{{hi:Embedded-hal-impl}}{{hi:Hal}}{{hi:Pi}}{{hi:Raspberry}} [![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]{{hi:Hardware support}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

[`rppal`][c~rppal~docs]↗{{hi:rppal}} is an interface for the Raspberry Pi's GPIO, I2C, PWM, SPI and UART peripherals.

## Related Topics {#related-topics .skip}

- [Peripheral Access Crates](pacs.md) provide lower-level, device-specific register access.
- [Sensors](sensors.md) use bus traits and device drivers to communicate with external hardware.
- [Real-Time Operating Systems](rtos.md) and async executors coordinate work across peripherals.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1203)
</div>
