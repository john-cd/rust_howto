# Embedded Frameworks/HALs (Hardware Abstraction Layers)

{{#include hals.incl.md}}

The [`embedded-hal`][c~embedded_hal~docs]↗{{hi:embedded-hal}} crate is the most important for writing portable embedded code. You'll then choose a HAL crate specific to your microcontroller family (e.g., `stm32fxxx-hal`). PACs give you direct register access, but you'll usually work with a HAL. RTOSs are used for more complex embedded applications. [`defmt`][c~defmt~docs]↗{{hi:defmt}} is a very useful logging crate.

## Key Crates {#skip}

- [`embedded-hal`][c~embedded_hal~docs]↗{{hi:embedded-hal}}: A crucial crate that defines common traits for interacting with peripherals (GPIO, SPI, I2C, UART, etc.). This crate is essential for writing portable embedded code.
- [`cortex-m`][c~cortex_m~docs]↗{{hi:cortex-m}}: Provides access to Cortex-M microcontroller peripherals.
- `stm32fxxx-hal`: HALs for specific STM32 microcontrollers. (Many microcontroller families have their own HAL crates.)
- [`nrf52-hal`][c~nrf52_hal~docs]↗{{hi:nrf52-hal}}: HALs for Nordic Semiconductor nRF52 microcontrollers.
- [`esp-hal`][c~esp_hal~docs]↗{{hi:esp-hal}}: HAL for Espressif chips.

Many other microcontroller families have their own HAL crates.

## `embedded-hal` {#embedded-hal}

[![embedded-hal][c~embedded_hal~docs~badge]][c~embedded_hal~docs] [![embedded-hal~crates.io][c~embedded_hal~crates.io~badge]][c~embedded_hal~crates.io] [![embedded-hal~github][c~embedded_hal~github~badge]][c~embedded_hal~github] [![embedded-hal~lib.rs][c~embedded_hal~lib.rs~badge]][c~embedded_hal~lib.rs]{{hi:embedded-hal}}{{hi:Hal}}{{hi:Io}} [![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}} [![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]{{hi:Hardware support}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}} [![cat~no-std][cat~no-std~badge]][cat~no-std]{{hi:No standard library}}

[`embedded-hal`][c~embedded_hal~docs]↗{{hi:embedded-hal}} is a Hardware Abstraction Layer (HAL) for embedded systems.

{{#example embedded-hal}}

## `rppal` {#skip}

[![rppal][c~rppal~docs~badge]][c~rppal~docs] [![rppal~crates.io][c~rppal~crates.io~badge]][c~rppal~crates.io] [![rppal~github][c~rppal~github~badge]][c~rppal~github] [![rppal~lib.rs][c~rppal~lib.rs~badge]][c~rppal~lib.rs]{{hi:rppal}}{{hi:Embedded-hal}}{{hi:Embedded-hal-impl}}{{hi:Hal}}{{hi:Pi}}{{hi:Raspberry}} [![cat~hardware-support][cat~hardware-support~badge]][cat~hardware-support]{{hi:Hardware support}} [![cat~embedded][cat~embedded~badge]][cat~embedded]{{hi:Embedded development}}

[`rppal`][c~rppal~docs]↗{{hi:rppal}} is an interface for the Raspberry Pi's GPIO, I2C, PWM, SPI and UART peripherals.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1203)
</div>
