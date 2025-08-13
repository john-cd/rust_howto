# OS - FreeBSD APIs

[![cat~os::freebsd-apis][cat~os::freebsd-apis~badge]][cat~os::freebsd-apis]{{hi:Operating systems::FreeBSD APIs}}

Bindings to FreeBSD-specific APIs.

Most standard Rust code that runs on other Unix-like systems (Linux, macOS) will work on FreeBSD without modification. For general system programming tasks (file I/O, networking, processes), the standard library is usually sufficient. If you need to use FreeBSD-specific system calls, FFI will be necessary. Be careful with [`unsafe`][keyword~unsafe]↗{{hi:unsafe}} code and consult the FreeBSD documentation.

## General and Unix-like System Calls

- File System: The standard library's [`std::fs`][c~std::fs~docs]↗{{hi:std::fs}} and [`std::path`][c~std::path~docs]↗{{hi:std::path}} modules work well on FreeBSD.
- Networking: The standard library's [`std::net`][c~std::net~docs]↗{{hi:std::net}} and crates like [`tokio`][c~tokio~docs]↗{{hi:tokio}} are compatible with FreeBSD.
- Processes and Threads: The standard library's [`std::process`][c~std::process~docs]↗{{hi:std::process}} and [`std::thread`][c~std::thread~docs]↗{{hi:std::thread}} work on FreeBSD.
- [`std::os::unix`][c~std::os::unix~docs]{{hi:std::os::unix}} provides access to many common Unix system calls. A large portion of these will work on FreeBSD.

## FreeBSD-Specific System Calls or Features

If you need to interact with FreeBSD-specific libraries or APIs that are written in C, you'll use Rust's [[development-tools_ffi | FFI]]. This involves declaring external functions and then calling them from your Rust code.

## Code Examples

{{#include freebsd.incl.md}}

## Related Topics

- [[os | OS]].
- [[os_linux-apis | OS: Linux APIs]].
- [[os_macos-apis | OS: macOS APIs]].
- [[os_unix-apis | OS: Unix APIs]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/947)
link to API bindings / External FFI cats in `crates.io`.
</div>
