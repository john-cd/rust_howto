# OS - Unix APIs

[![cat-os::unix-apis][cat-os::unix-apis-badge]][cat-os::unix-apis]

Bindings to Unix-specific APIs.{{hi:UNIX-specific APIs}}.

## File System Operations: Working with Files, Directories, Permissions, etc

- `std::fs`, `std::path`.

## Process Management: Forking, Spawning Processes, Signals, Pipes

- `std::process`, [`nix`][c-nix]⮳{{hi:nix}}.

## Networking (Sockets): TCP/IP, UDP, low-level Socket Programming

- `std::net`, [`socket2`][c-socket2]⮳{{hi:socket2}}.

## System Calls: Directly Interacting with the Operating System

- [`nix`][c-nix]⮳{{hi:nix}}, [`libc`][c-libc]⮳{{hi:libc}} (for raw system call access, use with caution).

## Terminal Interaction: Working with TTYs, Terminal Attributes

- [`termion`][c-termion]⮳{{hi:termion}}, [`crossterm`][c-crossterm]⮳{{hi:crossterm}}.

## Time and Dates: Working with System Time, Timers

- [`std::time`][c-std::time]⮳{{hi:std::time}}, [`chrono`][c-chrono]⮳{{hi:chrono}}.

## User and Group Management: Getting User/Group Information

- [`nix`][c-nix]⮳{{hi:nix}}.

## Handling POSIX Signals

- [`nix`][c-nix]⮳{{hi:nix}}, [`signal-hook`][c-signal_hook]⮳{{hi:signal-hook}}.

## Code Examples

{{#include unix.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/437)
convert into table?
</div>
