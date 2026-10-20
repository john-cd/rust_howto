# Generate FFI Bindings to Objective-C Code

{{#include objc.incl.md}}

Interfacing with Objective-C from Rust is typically done using the Objective-C runtime.

## `objc2` {#objc2}

[![objc2][c~objc2~docs~badge]][c~objc2~docs] [![objc2~crates.io][c~objc2~crates.io~badge]][c~objc2~crates.io] [![objc2~repo][c~objc2~repo~badge]][c~objc2~repo] [![objc2~lib.rs][c~objc2~lib.rs~badge]][c~objc2~lib.rs]{{hi:objc2}}

Objective-C bindings for Rust.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/objc/objc2.rs:example}}
```

## Build Tools {#build-tools .skip}

Use:

- [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building the Rust library.
- Xcode or the `clang` compiler for managing the Objective-C side.

## Key Concepts and Workflow {#key-concepts-and-workflow .skip}

- Objective-C Runtime: You'll write Rust code that uses the Objective-C runtime to interact with Objective-C classes and objects.
- Messaging: You'll send messages to Objective-C objects.
- Data Marshaling: You'll need to convert data between Rust types and Objective-C types (e.g., converting Rust [strings][p~strings] to NSStrings, and vice-versa).
- Memory Management: Objective-C uses reference counting. You'll need to be mindful of memory management to avoid leaks or crashes.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1075)
</div>
