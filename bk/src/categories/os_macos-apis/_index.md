# OS - MacOS APIs

[![cat-os::macos-apis][cat-os::macos-apis-badge]][cat-os::macos-apis]

Bindings to macOS-specific APIs.

{{#include macos.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[ P2 review](https://github.com/john-cd/rust_howto/issues/949)

Most standard Rust code will work on macOS without modification. For macOS-specific features, you'll need to use FFI and potentially the `objc` crate. Be careful with `unsafe` code and consult the Apple documentation. For general system programming tasks (file I/O, networking, processes), the standard library is often sufficient.

Rust code that adheres to standard practices will generally run well on macOS. However, if you need to interact with macOS-specific features, here's a breakdown:

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
- [`tokio`][c-tokio]⮳{{hi:tokio}}: For asynchronous networking. Essential for high-performance network applications on macOS.

## Threads

- `std::thread` works well on macOS.

## Time

- [`std::time`][c-std::time]⮳{{hi:std::time}} works well on macOS.
- [`chrono`][c-chrono]⮳{{hi:chrono}} or [`time`][c-time]⮳{{hi:time}}: For date and time calculations.

## macOS-Specific APIs (e.g., Cocoa, Objective-C)

For interacting with macOS-specific frameworks like Cocoa or Objective-C, you'll need to use Foreign Function Interface (FFI).

## Foreign Function Interface (FFI)

Rust's FFI allows you to call functions written in other languages (e.g., C, Objective-C) that interact with macOS directly. This is how you'll access macOS-specific APIs.

## Objective-C Runtime Interaction

- `objc`: A crate for interacting with the Objective-C runtime. This is essential if you need to work with Objective-C objects or frameworks.

## Key Considerations

- Portability: If you want your code to be portable across different operating systems, stick to the standard library and avoid macOS-specific APIs unless absolutely necessary.
- `unsafe` Code: Using FFI or interacting with the Objective-C runtime often involves `unsafe` code. Be very careful when working with `unsafe` code.
- Documentation: Consult the Apple developer documentation for details on macOS-specific APIs or frameworks.

</div>
