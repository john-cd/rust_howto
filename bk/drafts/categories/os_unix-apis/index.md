# OS - Unix APIs

[![cat~os::unix-apis][cat~os::unix-apis~badge]][cat~os::unix-apis]

Bindings to Unix-specific APIs.{{hi:UNIX-specific APIs}}.

## File System Operations: Working with Files, Directories, Permissions, etc

- [`std::fs`](https://doc.rust-lang.org/std/fs/index.html)↗{{hi:std::fs}}, [`std::path`](https://doc.rust-lang.org/std/path/index.html)↗{{hi:std::path}}.

## Process Management: Forking, Spawning Processes, Signals, Pipes

- `std::process`, [`nix`][c~nix~docs]↗{{hi:nix}}.

## Networking (Sockets): TCP/IP, UDP, low-level Socket Programming

- [`std::net`](https://doc.rust-lang.org/std/net/index.html)↗{{hi:std::net}}, [`socket2`][c~socket2~docs]↗{{hi:socket2}}.

## System Calls: Directly Interacting with the Operating System

- [`nix`][c~nix~docs]↗{{hi:nix}}, [`libc`][c~libc~docs]↗{{hi:libc}} (for raw system call access, use with caution).

## Terminal Interaction: Working with TTYs, Terminal Attributes

- [`termion`][c~termion~docs]↗{{hi:termion}}, [`crossterm`][c~crossterm~docs]↗{{hi:crossterm}}.

## Time and Dates: Working with System Time, Timers

- [`std::time`][c~std::time~docs]↗{{hi:std::time}}, [`chrono`][c~chrono~docs]↗{{hi:chrono}}.

## User and Group Management: Getting User/Group Information

- [`nix`][c~nix~docs]↗{{hi:nix}}.

## Handling POSIX Signals

- [`nix`][c~nix~docs]↗{{hi:nix}}, [`signal-hook`][c~signal_hook~docs]↗{{hi:signal-hook}}.

## Code Examples

{{#include unix.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/437)
convert into table?

- [extrasafe — Rust API for Unix](https://lib.rs/crates/extrasafe)

</div>
