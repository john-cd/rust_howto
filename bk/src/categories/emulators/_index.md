# Emulators

[![cat-emulators][cat-emulators-badge]][cat-emulators]{{hi:Emulators}}

Emulators allow one computer to behave like another, often to allow running software that is not natively available on the host computer. Video game systems are commonly emulated.

{{#include emulators.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[emulators/_index: write (P2)](https://github.com/john-cd/rust_howto/issues/348)

Building an emulator is a substantial project. You'll likely spend a lot of time on the core logic (CPU emulation, memory management) and then use crates for graphics, sound, and other peripherals as needed.

- CPU Emulation
- Memory Management
- Input/Output (I/O) Handling
- Debugging:
- ROM Loading and Parsing:
- Instruction Decoding:
- State Management:
- Concurrency (for performance):
- rayon: Can be used for parallelizing parts of the emulation.

Link to

## Graphics

`pixels`: A crate for working with pixel buffers, often used for rendering graphics.
`sdl2`: Bindings to the SDL library, which can be used for window management, input, and graphics.
`wgpu`: A crate for portable GPU compute.

## Sound

`cpal`: A crate for cross-platform audio input and output.

</div>
