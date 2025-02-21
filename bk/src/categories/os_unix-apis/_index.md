# OS - Unix APIs

[![cat-os::unix-apis][cat-os::unix-apis-badge]][cat-os::unix-apis]

Bindings to Unix-specific APIs.{{hi:UNIX-specific APIs}}.

{{#include unix.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[os_unix-apis/_index: review (P2)](https://github.com/john-cd/rust_howto/issues/437)

## File System Operations

Working with files, directories, permissions, etc.

- `std::fs`, `std::path`

## Process Management: Forking, spawning processes, signals, pipes

- `std::process`, `nix`

## Networking (Sockets): TCP/IP, UDP, low-level socket programming

- `std::net`, `socket2`

## System Calls: Directly interacting with the operating system

- `nix`, `libc` (for raw system call access, use with caution)

## Terminal Interaction: Working with TTYs, terminal attributes

- `termion`, `crossterm`

## Time and Dates: Working with system time, timers

- `std::time`, `chrono`

## User and Group Management

Getting user/group information.

- `nix`

## Signals: Handling POSIX signals

- `nix`, `signal-hook`

</div>
