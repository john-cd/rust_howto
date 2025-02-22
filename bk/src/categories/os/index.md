# Operating System

[![cat-os][cat-os-badge]][cat-os]

Bindings to operating system-specific APIs{{hi:Operating system-specific APIs}}.

## External

{{#include external_commands.incl.md}}

## Low-level system calls

{{#include low_level_system_calls.incl.md}}

## Operating Systems written in Rust

{{#include rust_os.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[os/index: fix (P1)](https://github.com/john-cd/rust_howto/issues/429)

Rust offers various ways to interact with the operating system, depending on your needs.

## Key Concepts

- System calls: Direct requests to the OS kernel for services.
- Processes: Running instances of programs.
- Threads: Lightweight units of execution within a process.
- File system: How files and directories are organized.
- Networking: How computers communicate with each other.

## Choosing Crates

- Basic OS interaction (files, processes, environment): [`std`][c-std]⮳{{hi:std}} (standard library).
- Asynchronous networking: [`tokio`][c-tokio]⮳{{hi:tokio}}.
- Date and time: [`chrono`][c-chrono]⮳{{hi:chrono}} or [`time`][c-time]⮳{{hi:time}}.
- System information: [`sysinfo`][c-sysinfo]⮳{{hi:sysinfo}}.
- Low-level OS interaction or access to specific OS features: Use system calls directly (via `std::os::unix` or `std::os::windows`) or FFI.

For most common OS interactions, the standard library ([`std`][c-std]⮳{{hi:std}}) will be sufficient. For high-performance networking, [`tokio`][c-tokio]⮳{{hi:tokio}} is essential. For date and time, [`chrono`][c-chrono]⮳{{hi:chrono}} or [`time`][c-time]⮳{{hi:time}} are good choices. For system information, use [`sysinfo`][c-sysinfo]⮳{{hi:sysinfo}}. For very specific or platform-dependent OS features, you might need to use system calls or FFI.

Here's a breakdown:

## System Calls (Low-Level OS Interaction)

- `std::os::unix` (Linux, macOS, other Unix-like): Provides access to Unix-specific system calls.
- `std::os::windows` (Windows): Provides access to Windows-specific system calls.

Link to:

## File System Interaction

- `std::fs` provides the core functionality for working with files and directories (opening, reading, writing, creating, deleting, etc.).
- `std::path` provides types and functions for working with file paths (`Path`, `PathBuf`).
- [`fs_extra`][c-fs_extra]⮳{{hi:fs_extra}} is a crate that provides additional file system operations.
- Use [`tempfile`][c-tempfile]⮳{{hi:tempfile}} for creating temporary files and directories.

## Process Management

- `std::process` for spawning and managing child processes.

## Environment Variables

- `std::env` for accessing and manipulating environment variables.

## Networking

- `std::net` provides basic networking functionality (TCP, UDP).
- [`tokio`][c-tokio]⮳{{hi:tokio}} is a powerful asynchronous runtime, essential for high-performance networking applications.

## Threads

- `std::thread` for creating and managing threads.

## Time

- [`std::time`][c-std::time]⮳{{hi:std::time}} for working with time and durations.
- [`chrono`][c-chrono]⮳{{hi:chrono}} is widely used crate for date and time calculations.
- [`time`][c-time]⮳{{hi:time}} is a newer crate for date and time.

## Random Number Generation

- [`rand`][c-rand]⮳{{hi:rand}}: A popular random number generator crate.

## Operating System Information

- [`sysinfo`][c-sysinfo]⮳{{hi:sysinfo}}: A crate for retrieving system information (CPU, memory, etc.).

## Foreign Function Interface (FFI)

Rust's FFI allows you to call functions written in other languages (e.g., C) that might interact with the OS directly. This is often used when you need to access OS features that aren't directly exposed by Rust's standard library or other crates.

</div>
