# OS - MacOS APIs

[![cat-os::macos-apis][cat-os::macos-apis-badge]][cat-os::macos-apis]

Bindings to macOS-specific APIs.

Most standard Rust code will work on macOS without modification. For general system programming tasks (file I/O, networking, processes), the standard library is often sufficient. In fact, if you want your code to be portable across different operating systems, stick to the standard library and avoid macOS-specific APIs unless absolutely necessary.

For macOS-specific features, you'll need to use the [`objc`][c-objc]⮳{{hi:objc}} crate and potentially FFI. Be careful with `unsafe` code and consult the Apple documentation.

## System Calls (Low-Level OS Interaction)

- `std::os::unix`: macOS is a Unix-like operating system, so a large portion of the Unix system calls provided by `std::os::unix` will work correctly.
- [`nix`][c-nix]⮳{{hi:nix}}: A crate providing more comprehensive access to Unix-like system calls. It is often preferred over using `std::os::unix` directly.

## File System Interaction

- `std::fs` works well on macOS.
- `std::path` works well on macOS.

## Process Management

- `std::process` works well on macOS.

## Networking

- `std::net` works well on macOS.
- For asynchronous networking, use [`tokio`][c-tokio]⮳{{hi:tokio}}.

## Threads

- `std::thread` works well on macOS.

## Time

- [`std::time`][c-std::time]⮳{{hi:std::time}} works well on macOS.
- Use [`chrono`][c-chrono]⮳{{hi:chrono}} or [`time`][c-time]⮳{{hi:time}} for date and time calculations.

## Objective-C Runtime Interaction

If you need to work with Objective-C objects or frameworks, consider the [`objc`][c-objc]⮳{{hi:objc}} crate.
See [[objc | Objc]].

## Other macOS-specific APIs

For interacting with macOS-specific frameworks, first review the [[api-bindings | API Bindings]] category on 'crates.io'.

If no safe wrapper exists for the desired functionality, you may need to use Foreign Function Interface (FFI).
See [[development-tools_ffi | Development Tools: FFI]] and [[external-ffi-bindings | External FFI Bindings]].

## Code Examples

{{#include macos.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/949)
review in depth
interface with `Cocoa`
`Swift` bindings
</div>
