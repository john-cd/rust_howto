# Generate FFI bindings to Objective-C code

{{#include objc.incl.md}}

## `objc2` {#objc2}

[![objc2][c-objc2-badge]][c-objc2] [![objc2-crates.io][c-objc2-crates.io-badge]][c-objc2-crates.io] [![objc2-github][c-objc2-github-badge]][c-objc2-github] [![objc2-lib.rs][c-objc2-lib.rs-badge]][c-objc2-lib.rs]{{hi:objc2}}{{hi:Ios}}{{hi:Objective-c}}{{hi:Macos}}{{hi:Objc}}{{hi:Objc_msgsend}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}} [![cat-development-tools::ffi][cat-development-tools::ffi-badge]][cat-development-tools::ffi]{{hi:FFI}} [![cat-external-ffi-bindings][cat-external-ffi-bindings-badge]][cat-external-ffi-bindings]{{hi:External FFI bindings}} [![cat-os::macos-apis][cat-os::macos-apis-badge]][cat-os::macos-apis]{{hi:macOS APIs}}

[`objc2`][c-objc2]⮳{{hi:objc2}} provides an Objective-C interface and runtime bindings.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/objc/objc2.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P2 write](https://github.com/john-cd/rust_howto/issues/1075)

Interfacing with Objective-C from Rust is primarily done using the objc crate and related tools. Here's a breakdown:

## Objective-C Runtime Bindings

objc: This is the core crate for interacting with the Objective-C runtime. It provides access to Objective-C classes, methods, and objects. It handles a lot of the low-level details of Objective-C interaction.

## Code Generation (Often Needed)

objc_foundation: Provides bindings for the Foundation framework classes (NSString, NSArray, etc.). Often used in conjunction with objc.

Bridging the Gap (Manual or Semi-Automated): There isn't a single perfect tool for fully automating the bridging process like some other language bindings. It often involves a combination of:

cbindgen: Can be used to generate C header files from your Rust code if you need to create a C interface as an intermediary.
Manual coding: You'll often need to write some glue code to convert between Rust types and Objective-C types.
Build Tools:

cargo: (For building the Rust library.)
Xcode: (For building the Objective-C/Swift side and integrating with the Rust library. You'll typically create a framework or library that includes your Rust code.)

## Key Concepts and Workflow

Objective-C Runtime: You'll use the objc crate to interact with the Objective-C runtime.
Messaging: You'll send messages to Objective-C objects using the objc crate.
Data Marshaling: You'll need to convert data between Rust types and Objective-C types (e.g., converting Rust strings to NSStrings, and vice-versa).
Memory Management: Objective-C uses manual reference counting (ARC). You'll need to be mindful of memory management to avoid leaks or crashes. The objc crate provides tools to help with this.
Error Handling: You'll need to handle Objective-C exceptions in your Rust code.
Swift Interop (Important Note): If you're working with Swift, you'll likely interact with Objective-C as an intermediary, as Swift has excellent Objective-C interoperability. So, the workflow is often: Rust -> Objective-C -> Swift.

The objc crate is the most important crate for Objective-C interaction. Be prepared for some manual work and glue code, especially when dealing with data marshaling and memory management. Objective-C interoperability with Rust is more complex than some other language bindings due to the nature of the Objective-C runtime and ARC.
</div>
