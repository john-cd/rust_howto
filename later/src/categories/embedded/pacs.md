<!-- TODO Reviewed ??-??-2026 -->

# Peripheral Access Crates (PACs)

{{#include pacs.incl.md}}

Peripheral Access Crates (PACs) expose a microcontroller's memory-mapped registers, interrupt identifiers, and peripheral ownership tokens. Most are generated from a vendor's System View Description (SVD) file, often with [`svd2rust`][svd2rust~repo]↗. A PAC is specific to a chip or closely related family: select the exact part and enable only its supported device feature.

## Using a PAC {#using-a-pac}

The PAC gives low-level code typed access to register blocks. Many device PACs expose a `Peripherals::take()` method that returns the peripheral singleton tokens once; the tokens let safe code transfer ownership of a peripheral instead of creating multiple owners. Keep this ownership model intact. Raw pointers or direct register access can bypass it and require a narrow, audited `unsafe` boundary.

A PAC is not a complete board configuration. It may describe register fields and interrupt names, but application code still has to configure clocks, pin multiplexing, reset state, and electrical connections. A chip HAL normally builds on the PAC and provides safer, higher-level operations for common peripherals.

## PAC or HAL?

| Use a PAC when                                                                                               | Use a HAL when                                                                            |
| ------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------- |
| A required register or interrupt feature is not exposed by the HAL.                                          | You need ordinary GPIO, I2C, SPI, PWM, timer, or serial operations.                       |
| You are implementing or debugging a HAL or device-specific driver.                                           | You want application code or a portable driver to target a stable trait interface.        |
| You need precise control over a register sequence and can verify it against the reference manual and errata. | You want the HAL to own peripheral configuration and enforce pin or peripheral ownership. |

If using registers directly, check the reference manual for access width, read/write side effects, reset values, reserved bits, and required ordering. Do not assume a PAC proves that an operation is electrically safe or valid for the board.

## Choosing and Updating a PAC

Confirm the exact chip revision and crate feature, then read the PAC documentation for its generated register map and interrupt names. SVD data can be incomplete or differ from vendor documentation; compare consequential operations with the device reference manual and errata. When a PAC is generated or updated, review the SVD source and generated diff before relying on new register descriptions.

The [`cortex-m`][c~cortex-m~docs]↗ crate describes Cortex-M core peripherals. It does not replace the chip PAC, which describes the vendor's GPIO, timers, serial controllers, and other device-specific blocks.

## Related Topics {#related-topics .skip}

- [Hardware Abstraction Layers](hals.md) usually build on a PAC and provide higher-level peripheral APIs.
- [Sensors](sensors.md) are normally accessed through a HAL bus implementation and a device driver.
- [Flash programming](flash.md) uses device-specific memory maps and programming algorithms.
- Bare-metal and [no-std development](../no-std/index.md) provide the surrounding runtime context.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  write
</div>
