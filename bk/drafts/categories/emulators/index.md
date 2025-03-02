# Emulators

[![cat-emulators][cat-emulators-badge]][cat-emulators]{{hi:Emulators}}

Emulators allow one computer to behave like another, often to allow running software that is not natively available on the host computer. Video game systems are commonly emulated.

Please also review [[virtualization | Virtualization]].

{{#include emulators.incl.md}}

## Building an emulator

Building an emulator is a substantial project. You'll likely spend a lot of time on the core logic (CPU emulation, memory management) and then use crates for graphics, sound, and other peripherals as needed.

## CPU Emulation and Instruction Decoding

See [[hardware-support | Hardware Support]], [[parsing | Parsing]], [[parser-implementations | Parser Implementations]], and
[[_binary_encoders |  Binary Encoders]].

## Input/Output (I/O) Handling

See [[filesystem | Filesystem]].

## Memory Management

See [[memory-management | Memory Management]].

### ROM Loading and Parsing

See [[parser-implementations | Parser Implementations]].

## Graphics

Consider using:

- [`pixels`][c-pixels]⮳{{hi:pixels}}: A crate for working with pixel buffers, often used for rendering graphics.
- [`sdl2`][c-sdl2]⮳{{hi:sdl2}}: Bindings to the SDL library, which can be used for window management, input, and graphics.
- [`wgpu`][c-wgpu]⮳{{hi:wgpu}}: A crate for portable GPU compute.

See [[graphics | Graphics]].

## Sound

Consider [`cpal`][c-cpal]⮳{{hi:cpal}}, a crate for cross-platform audio input and output. See [[audio | Audio]].

## Concurrency

`rayon` can be used for parallelizing parts of the emulation (for performance). See [[concurrency | Concurrency]].

## Tooling

### Debugging

See [[development-tools_debugging | Development Tools: Debugging]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write (P2)](https://github.com/john-cd/rust_howto/issues/348)

</div>
