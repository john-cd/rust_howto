<!-- TODO Reviewed ??-??-2026 -->

# Emulators

[![cat~emulators][cat~emulators~badge]][cat~emulators]{{hi:Emulators}}

Emulators reproduce another system's behavior so its software can run in a different environment. Implementations vary with the target's CPU, memory map, timing, and peripherals.

Use the [Emulators chapter][ex~emulators~emulators] for the instruction interpreter and ROM-loading example. For broader context, review [[virtualization | Virtualization]].

{{#include emulators.incl.md}}

## Related Topics

- For display output, see [`pixels`][c~pixels~docs] or [`minifb`][c~minifb~docs].
- For audio output, see [`cpal`][c~cpal~docs] and [[audio | Audio]].
- For diagnostics, see [[development-tools_debugging | Development Tools: Debugging]].
- For instruction sets and binary data, see [[hardware-support | Hardware Support]] and [[parsing | Parsing]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
