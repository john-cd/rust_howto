# OS - Linux APIs

[![cat~os::linux-apis][cat~os::linux-apis~badge]][cat~os::linux-apis]

Bindings to Linux-specific APIs.

Most standard Rust code will work on Linux without modification. For general system programming tasks, the standard library is often sufficient. For Linux-specific features, you might need to use [`nix`][c~nix~docs]⮳{{hi:nix}}, [[development-tools_ffi | FFI]], or find crates that provide bindings to the specific interface you're working with. Be careful with `unsafe` code and consult the Linux documentation.

## System Calls (Low-Level OS Interaction)

- `std::os::unix`: This module provides access to a large number of Unix system calls, which are generally the same or very similar across Linux and other Unix-like systems. This is your primary way to interact with the Linux kernel directly.

- [`nix`][c~nix~docs]⮳{{hi:nix}} provides more comprehensive access to Unix-like system calls, including many Linux-specific ones. It is often preferred over using `std::os::unix` directly for better type safety and error handling.

## File System Interaction

- `std::fs`: (Standard library) For working with files and directories. Works well on Linux.
- `std::path`: (Standard library) For working with file paths. Works well on Linux.

## Process Management

- `std::process`: spawning and managing child processes. Works well on Linux.

## Networking

- `std::net`: basic networking (TCP, UDP). Works well on Linux.
- [`tokio`][c~tokio~docs]⮳{{hi:tokio}}: asynchronous networking.

## Threads

- `std::thread`: creating and managing threads. Works well on Linux.

## Time

- [`std::time`][c~std::time~docs]⮳{{hi:std::time}}: working with time and durations. Works well on Linux.
- Also use [`chrono`][c~chrono~docs]⮳{{hi:chrono}} or [`time`][c~time~docs]⮳{{hi:time}} for date and time calculations.

## Linux-Specific Features (e.g., `systemd`, Specific Kernel interfaces)

For interacting with Linux-specific services or APIs, you'll often need to find or create crates that provide bindings to those specific interfaces.

See [[api-bindings | API Bindings]] and [[external_ffi_bindings | External FFI Bindings]].

## Foreign Function Interface (FFI)

Rust's FFI allows you to call functions written in other languages (e.g., C) that might interact with Linux directly. This is often used when you need to access OS features that aren't directly exposed by Rust's standard library or other crates.

See [[development-tools_ffi | Development Tools FFI]].

## Device Drivers (Kernel-Level)

Writing device drivers is advanced and requires close interaction with the Linux kernel. You'll typically need to write kernel modules in C and then use FFI from Rust user-space programs.

## Code Examples

{{#include linux.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/948)
</div>
