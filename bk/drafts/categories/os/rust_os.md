<!-- TODO Reviewed ??-??-2026 -->

# Operating Systems Written in Rust

{{#include rust_os.incl.md}}

## Host Containers with `bottlerocket` {#bottlerocket}

[![bottlerocket][c~bottlerocket~docs~badge]][c~bottlerocket~docs] [![bottlerocket~crates.io][c~bottlerocket~crates.io~badge]][c~bottlerocket~crates.io] [![c~bottlerocket~repo][c~bottlerocket~repo~badge]][c~bottlerocket~repo] [![bottlerocket~lib.rs][c~bottlerocket~lib.rs~badge]][c~bottlerocket~lib.rs]{{hi:bottlerocket}}

[`bottlerocket`][c~bottlerocket~repo]↗{{hi:bottlerocket}} is an operating system designed for hosting containers.

## Run a Rust Operating System on your Computer with `Redox` {#redox}

[`Redox`][redox~website]↗[(github)][redox~repo]↗{{hi:redox}} is an experimental, Unix-like, general-purpose, microkernel-based operating system written in Rust, aiming to bring Rust to a modern microkernel, a full set of programs, and be a complete alternative to Linux and BSD.

## Write an Operating System Kernel in Rust {#write-a-kernel}

A kernel runs without the standard library, so start from a `#![no_std]` and `#![no_main]` crate that targets bare metal. A custom target specification or an existing bare-metal target such as `x86_64-unknown-none` replaces the host target, and the crate supplies its own entry point and panic handler [blog_os~guide]. The ["Writing an OS in Rust"][blog_os~guide] series walks through this path step by step.

Several crates remove the lowest-level boilerplate:

- [`bootloader`][bootloader~repo] builds a bootable disk image and hands the kernel a boot information structure.
- [`x86_64`][x86_64~repo] wraps CPU instructions, page tables, and interrupt descriptor tables.
- [`uefi`][uefi~repo] lets a Rust program run as a UEFI application and call firmware services.

## Use Rust in the Linux Kernel {#linux}

Linux accepts drivers and subsystems written in Rust alongside C. The kernel documentation describes the toolchain requirements and the safe abstractions that wrap kernel APIs [linux~rust-docs].

## More Operating Systems Written in Rust {#more}

[`Tock`][tock~website]↗ targets secure embedded systems. More operating systems and Rust-for-Linux appear on the [Written in Rust][p~written-in-rust] page.

## Related Topics {#related-topics .skip}

- [[low_level_system_calls | Low-level System Calls]].
- [[process_management | Process Management]].
- [Written in Rust][p~written-in-rust].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
