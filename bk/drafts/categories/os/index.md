# Operating System

[![cat~os][cat~os~badge]][cat~os]

Bindings to operating system-specific APIs{{hi:Operating system-specific APIs}}.

Rust offers various ways to interact with the operating system, depending on your needs. For most common OS interactions, the standard library ([`std`][c~std~docs]↗{{hi:std}}) will be sufficient. For system information, use [`sysinfo`][c~sysinfo~docs]↗{{hi:sysinfo}}. For very specific or platform-dependent OS features, you might need to use system calls or FFI.

## External Commands

{{#include external_commands.incl.md}}

## Low-level System Calls

{{#include low_level_system_calls.incl.md}}

## Operating Systems Written in Rust

{{#include rust_os.incl.md}}

## Related Topics

| Topic | Rust Crates |
|---|---|
| Basic OS interaction (files, processes, environment) | Use [`std`][c~std~docs]↗{{hi:std}}, the standard library. |
| File System Interaction | [`std::fs`][c~std::fs~docs]↗{{hi:std::fs}} provides the core functionality for working with files and directories (opening, reading, writing, creating, deleting, etc.). [`std::path`][c~std::path~docs]↗{{hi:std::path}} provides types and functions for working with file paths (`Path`{{hi:Path}}, `PathBuf`{{hi:PathBuf}}). [`fs_extra`][c~fs_extra~docs]↗{{hi:fs_extra}} is a crate that provides additional file system operations. Use [`tempfile`][c~tempfile~docs]↗{{hi:tempfile}} for creating temporary files and directories. |
| Process Management | Use [`std::process`]( )↗{{hi: }} for spawning and managing child processes. |
| Environment Variables | Use [`std::env`][c~std::env~docs]↗{{hi:std::env}} for accessing and manipulating environment variables. |
| Networking | [`std::net`][c~std::net~docs]↗{{hi:std::net}} provides basic networking functionality (TCP, UDP). [`tokio`][c~tokio~docs]↗{{hi:tokio}} is a powerful asynchronous runtime, essential for high-performance networking applications. |
| Threading | Use [`std::thread`]( )↗{{hi: }} for creating and managing threads. |
| Date and time | Use [`std::time`][c~std::time~docs]↗{{hi:std::time}} for working with time and durations. [`chrono`][c~chrono~docs]↗{{hi:chrono}} is widely used crate for date and time calculations. [`time`][c~time~docs]↗{{hi:time}} is a newer crate for date and time. |
| Operating System Information | Use [`sysinfo`][c~sysinfo~docs]↗{{hi:sysinfo}} to retrieve system information (CPU, memory, etc.). |
| Low-level OS interaction or access to specific OS features | Use system calls directly (via [`std::os::unix`](https://doc.rust-lang.org/std/os/unix)↗{{hi:std::os::unix}} or [`std::os::windows`](https://doc.rust-lang.org/std/os/windows)↗{{hi:std::os::windows}}) or FFI. `std::os::unix` provides access to Unix-specific system calls. `std::os::windows` provides access to Windows-specific system calls. |
| Foreign Function Interface (FFI) | Rust's FFI allows you to call functions written in other languages (e.g., C) that might interact with the OS directly. This is often used when you need to access OS features that aren't directly exposed by Rust's standard library or other crates. |
| Random Number Generation | [`rand`][c~rand~docs]↗{{hi:rand}} is a popular random number generator crate, which can in turn call the OS. |

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[os/index: fix; organize; add cross links](https://github.com/john-cd/rust_howto/issues/429)
</div>
