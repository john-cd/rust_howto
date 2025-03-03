# OS - FreeBSD APIs

[![cat-os::freebsd-apis][cat-os::freebsd-apis-badge]][cat-os::freebsd-apis]{{hi:Operating systems::FreeBSD APIs}}

Bindings to FreeBSD-specific APIs.

{{#include freebsd.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/947)

Most standard Rust code will work on FreeBSD without modification. If you need to use FreeBSD-specific system calls, the [`syscall`][c-syscall]⮳{{hi:syscall}} crate or FFI will be necessary. Be careful with `unsafe` code and consult the FreeBSD documentation. For general system programming tasks (file I/O, networking, processes), the standard library is usually sufficient.

## Key Considerations

- Portability: If you want your code to be portable across different Unix-like systems, stick to the standard library and avoid FreeBSD-specific system calls unless absolutely necessary.
- `unsafe` Code: Using the [`syscall`][c-syscall]⮳{{hi:syscall}} crate or FFI often involves `unsafe` code.Be very careful when working with `unsafe` code, as it can lead to memory safety issues.

Rust code that runs on other Unix-like systems (Linux, macOS) will generally work on FreeBSD with little to no modification, as long as it's not doing anything highly platform-specific. However, if you're interacting with FreeBSD-specific features, here's a breakdown:

## General Unix-like System Calls (Often Work on FreeBSD)

- `std::os::unix`: (Standard library) This module provides access to many common Unix system calls.A large portion of these will work on FreeBSD.

## FreeBSD-Specific System Calls or Features

If you need to use a FreeBSD-specific system call not covered by `std::os::unix`, you'll likely use the [`syscall`][c-syscall]⮳{{hi:syscall}} crate or FFI.

- [`syscall`][c-syscall]⮳{{hi:syscall}}:This crate allows you to make raw system calls.You would use this if there's a FreeBSD-specific system call that isn't already wrapped by the standard library.Be careful with this, as it's inherently `unsafe`.

- FFI (Foreign Function Interface):If you need to interact with FreeBSD-specific libraries or APIs that are written in C, you'll use Rust's FFI.This involves declaring external functions and then calling them from your Rust code.

## Other Relevant Topics

- File System: The standard library's `std::fs` and `std::path` modules work well on FreeBSD.
- Networking: The standard library's `std::net` and crates like [`tokio`][c-tokio]⮳{{hi:tokio}} are compatible with FreeBSD.
- Processes and Threads: The standard library's `std::process` and `std::thread` work on FreeBSD.
- Build Tools: [`cargo`][c-cargo]⮳{{hi:cargo}} works seamlessly on FreeBSD.

</div>
