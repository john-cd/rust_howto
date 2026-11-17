<!-- TODO Reviewed ??-??-2026. -->

# Operating Systems Written in Rust

{{#include rust_os.incl.md}}

## Bottlerocket for Container Hosts {#bottlerocket}

[![bottlerocket][c~bottlerocket~docs~badge]][c~bottlerocket~docs] [![bottlerocket~crates.io][c~bottlerocket~crates.io~badge]][c~bottlerocket~crates.io] [![c~bottlerocket~repo][c~bottlerocket~repo~badge]][c~bottlerocket~repo] [![bottlerocket~lib.rs][c~bottlerocket~lib.rs~badge]][c~bottlerocket~lib.rs]{{hi:bottlerocket}}

[`bottlerocket`][c~bottlerocket~repo]↗{{hi:bottlerocket}} is an operating system designed for hosting containers.

## Explore the Redox Operating System {#redox}

[`Redox`][redox~website]↗ ([source][redox~repo]↗) is an experimental, Unix-like operating system written in Rust.

## Write an Operating System Kernel in Rust {#write-a-kernel}

A kernel runs without the standard library, so start from a `#![no_std]` and `#![no_main]` crate that targets bare metal. A custom target specification or an existing bare-metal target such as `x86_64-unknown-none` replaces the host target, and the crate supplies its own entry point and panic handler. The [Writing an OS in Rust][blog_os~guide] series walks through this path step by step.

Several crates remove the lowest-level boilerplate:

- [`bootloader`][bootloader~repo] builds a bootable disk image and hands the kernel a boot information structure.
- [`x86_64`][x86_64~repo] wraps CPU instructions, page tables, and interrupt descriptor tables.
- [`uefi`][uefi~repo] lets a Rust program run as a UEFI application and call firmware services.

## Use Rust in the Linux Kernel {#linux}

Linux accepts drivers and subsystems written in Rust alongside C. The kernel documentation describes the toolchain requirements and the safe abstractions that wrap kernel APIs [linux~rust-docs].

## More Operating Systems Written in Rust {#more}

[`Tock`][tock~website]↗ targets secure embedded systems. The [Rust OSDev organization](https://github.com/rust-osdev) hosts more operating-system projects and kernel components.

## Related Topics {#related-topics .skip}

- [Low-level System Calls](low_level_system_calls.md).
- [Process Management](process_management.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
