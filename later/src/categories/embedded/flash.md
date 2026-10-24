<!-- TODO Reviewed ??-??-2026 -->

# Flash Programming

{{#include flash.incl.md}}

"Flash programming" can mean two different tasks: loading firmware onto a target during development or boot, and storing application data while the firmware runs. The first uses a programmer or debug probe; the second needs a storage layout and code that follows the memory device's erase and write rules.

## Program Firmware with a Debug Probe {#program-firmware}

Tools such as [`probe-rs`][c~probe-rs~docs]↗{{hi:probe-rs}} connect to a supported target through a debug probe, identify its chip, and program an ELF, Intel HEX, or binary image using the target's flash algorithm. Confirm the exact chip and probe, build for the correct target, and use the tool's documented target and permission settings. A successful download confirms that programming completed; still verify that the firmware boots and that its configuration matches the board.

Keep bootloader, application, metadata, and reserved regions within the memory map. A binary file may not carry address information, so check its load address before using it. Never write an application image over a bootloader or persistent-data partition by assuming that the entire chip is available.

## Store Application Data in NOR Flash {#application-storage}

Runtime storage has stricter semantics than a byte array. NOR flash is erased in blocks, writes must satisfy device-specific alignment, and programming usually changes bits from `1` to `0`; changing a `0` back to `1` requires erasing an entire erase block first. Read, write, and erase sizes differ between devices, so query and follow the selected flash driver's documented constraints.

The [`embedded-storage`][c~embedded-storage~docs]↗ traits describe common operations for storage devices, including read-only and writable NOR flash. They make storage code reusable across compatible devices; they do not remove the need to choose a partition, respect alignment, or manage wear and interruption.

The following example implements those traits over a small in-memory NOR model. It runs on the host without a flash device and demonstrates erase-block alignment and the one-to-zero programming rule; the model is not a substitute for validating a driver's geometry on the target.

{{#include ../../../crates/cats/embedded/examples/flash/main.rs:example}}

For persistent records, define a format with versioning and integrity checks. Design updates so power loss cannot leave the only copy unusable; common approaches include append-only records, two-slot updates, checksums, and a bootloader-managed update state. Avoid frequently erasing a single block for counters or logs, since flash endurance is finite.

## Validate the Storage Design

- Reserve explicit address ranges for firmware, bootloader, settings, and logs; check that build-time and runtime layouts agree.
- Check capacity, erase-block size, read/write alignment, and address bounds before each operation.
- Decide what happens after a reset or power failure during erase or write, and retain a recoverable previous record where required.
- Test the storage algorithm against a flash model that enforces erase-before-write semantics, then verify the final behavior on the target device.
- Keep firmware flashing and application-level data updates as distinct operations with separate permissions and recovery procedures.

## Related Topics {#related-topics .skip}

- [Peripheral Access Crates](pacs.md) describe the target register map.
- [Hardware Abstraction Layers](hals.md) provide the application-facing peripheral APIs.
- [Embedded systems](index.md) cover firmware setup and device constraints.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1200)
</div>
