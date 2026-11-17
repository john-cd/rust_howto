<!-- TODO Reviewed ??-??-2026. -->

# Hardware Support

This crate contains code examples for the **Hardware Support** category on crates.io. It uses `clap` for the SIMD example runner, `num_cpus` for CPU enumeration, `wide` for portable SIMD, and `serialport` for device enumeration.

## Examples Covered

- **Processor** (`examples/processor/`): Demonstrates concepts like `cpu_count`.
- **Simd** (`examples/simd/`): Demonstrates concepts like `simd_arch`.
- **Peripheral Devices** (`examples/peripherals/`): Lists serial ports reported by the operating system without opening them.

## Usage

To check or run examples in this crate:

```console
cargo check --package hardware_support --examples
cargo test --package hardware_support
```

Code examples correspond to chapters in `bk/src/categories/hardware-support` or related book sections.
