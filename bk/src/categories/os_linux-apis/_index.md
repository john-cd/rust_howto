# OS - Linux APIs

[![cat-os::linux-apis][cat-os::linux-apis-badge]][cat-os::linux-apis]

Bindings to Linux-specific APIs.

{{#include linux.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/948)

Most standard Rust code will work on Linux without modification. For Linux-specific features, you might need to use `nix`, FFI, or find crates that provide bindings to the specific interface you're working with. Be careful with `unsafe` code and consult the Linux documentation. For general system programming tasks, the standard library is often sufficient.

Rust code that adheres to standard practices will generally run seamlessly on Linux. However, if you need to interact with Linux-specific features, here's a breakdown:

## System Calls (Low-Level OS Interaction)

- `std::os::unix`: (Standard library) This module provides access to a large number of Unix system calls, which are generally the same or very similar across Linux and other Unix-like systems. This is your primary way to interact with the Linux kernel directly.

- `nix`: A crate providing more comprehensive access to Unix-like system calls, including many Linux-specific ones. It is often preferred over using `std::os::unix` directly for better type safety and error handling.

## File System Interaction

- `std::fs`: (Standard library) For working with files and directories. Works well on Linux.
- `std::path`: (Standard library) For working with file paths. Works well on Linux.

## Process Management

- `std::process`: (Standard library) For spawning and managing child processes. Works well on Linux.

## Networking

- `std::net`: (Standard library) For basic networking (TCP, UDP). Works well on Linux.
- `tokio`: For asynchronous networking. Essential for high-performance network applications on Linux.

## Threads

- `std::thread`: (Standard library) For creating and managing threads. Works well on Linux.

## Time

- `std::time`: (Standard library) For working with time and durations. Works well on Linux.
- `chrono` or `time`: For date and time calculations.

## Device Drivers (Kernel-Level)

Writing device drivers is advanced and requires close interaction with the Linux kernel. You'll typically need to write kernel modules in C and then use FFI from Rust user-space programs.

## Linux-Specific Features (e.g., systemd, specific kernel interfaces)

For interacting with Linux-specific services or APIs, you'll often need to find or create crates that provide bindings to those specific interfaces.

## Foreign Function Interface (FFI)

Rust's FFI allows you to call functions written in other languages (e.g., C) that might interact with Linux directly. This is often used when you need to access OS features that aren't directly exposed by Rust's standard library or other crates.

</div>
