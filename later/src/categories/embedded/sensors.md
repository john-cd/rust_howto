<!-- TODO Reviewed ??-??-2026 -->

# Communicating with Sensors over I2C, SPI, and UART

{{#include sensors.incl.md}}

An external sensor usually exposes a small register map or a byte-oriented protocol over I2C, SPI, or UART. A driver turns those transfers into operations with device-specific names and units. Prefer an existing driver crate when it supports your sensor and the bus traits implemented by your HAL; write a low-level driver only when you need to support a device that has no suitable driver.

## Choose the Bus and Driver

| Bus | What to configure | Common checks |
| --- | --- | --- |
| I2C | 7-bit address, bus speed, and any required pull-up resistors. | Confirm the address and voltage levels; check for a NACK and whether the device requires a combined write/read transaction. |
| SPI | Clock mode, bit order, clock rate, and chip-select behavior. | Confirm the mode and maximum clock rate; keep the correct chip select active for the complete transaction. |
| UART | Baud rate, data bits, parity, stop bits, and framing. | Confirm message boundaries, checksums, and how the receiver resynchronizes after malformed input. |

The bus wiring and the software configuration must agree with the sensor data sheet. Check logic voltage, pin muxing, pull-ups, reset timing, and any required startup delay before debugging a driver.

## Read a Register over I2C {#read-a-register-over-i2c}

The example uses `embedded-hal`'s combined write/read operation with a host-side mock sensor. Its address, register, byte order, and conversion formula are illustrative; use the device data sheet and replace the mock bus with the target HAL's I2C implementation. It builds without an I2C controller or attached sensor.

{{#include ../../../crates/cats/embedded/examples/sensors/main.rs:example}}

`write_read` sends the register selection and reads the response as one logical transaction. Some devices require a delay, a separate stop condition, or a different byte order; follow the device protocol rather than assuming every register map behaves alike. The [`embedded-hal`][c~embedded-hal~docs]↗ traits let a driver use an I2C implementation supplied by the target HAL.

## Decode and Schedule Measurements

Treat wire values as bytes until the protocol is decoded. Check signedness, byte order, reserved bits, scaling, and units before converting a measurement to a floating-point or domain value. Apply calibration with documented coefficients and keep raw values available when it helps diagnose a sensor.

Respect the device's conversion time and data-ready behavior. Sampling faster than a sensor can update only repeats stale readings and can occupy the bus. In an async or real-time system, avoid a long blocking transfer in a task that must service other deadlines; use an async driver or schedule the work at a suitable priority when the HAL supports it.

## Share the Bus Safely

Several devices may share an I2C bus. SPI devices share clock and data lines but each needs a chip-select signal and correct transaction boundaries. Use a bus-sharing adapter or the RTOS's synchronization mechanism so two drivers do not interleave operations. Keep bus locks short and define how the application handles timeouts, disconnected devices, and invalid samples.

See [Hardware Abstraction Layers](hals.md), [Peripheral Access Crates](pacs.md), and [Real-Time Operating Systems](rtos.md) for the lower-level interfaces and scheduling models.

## Related Topics {#related-topics .skip}

- Embedded communication protocols.
- Sensor fusion and control loops.
- Microcontroller peripheral access.
- Robotics and IoT applications.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1199)
</div>
