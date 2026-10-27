<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Objective-C Code

{{#include objc.incl.md}}

Interfacing with Objective-C from Rust is typically done using the Objective-C runtime.

## `objc2` {#objc2}

[![objc2][c~objc2~docs~badge]][c~objc2~docs] [![objc2~crates.io][c~objc2~crates.io~badge]][c~objc2~crates.io] [![objc2~repo][c~objc2~repo~badge]][c~objc2~repo] [![objc2~lib.rs][c~objc2~lib.rs~badge]][c~objc2~lib.rs]{{hi:objc2}}

Objective-C bindings for Rust.

Use `objc2` with its [generated framework crates][ffi~objc-frameworks]. `objc2-foundation` provides types such as `NSString`; `objc2-app-kit` provides macOS UI APIs. Import framework types rather than reconstructing all methods with raw `msg_send!`. The older `objc` and `cocoa` crates are superseded for this workflow by `objc2` and its framework crates.

The example uses the typed Foundation string API and `Retained<NSString>` ownership. It runs only on Apple targets with this repository's `objc` feature enabled. On other targets it compiles a fallback, so a passing Windows or Linux test is not evidence that Apple framework calls executed.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/objc/objc2.rs:example}}
```

## Build Tools {#build-tools .skip}

Use:

- [`cargo`][c~cargo~docs]↗{{hi:cargo}} for building the Rust library.
- Xcode or the `clang` compiler for managing the Objective-C side.

From `later/crates`, run `cargo test --locked -p development_tools_ffi --example objc --features objc` on macOS to exercise the string conversion. Use a macOS runner with the appropriate SDK; cross-compiling alone does not exercise runtime behavior. iOS packaging also needs the device/simulator target, signing configuration, and an application host. Keep AppKit calls on macOS, rather than assuming they are available on every Apple platform.

## Key Concepts and Workflow {#key-concepts-and-workflow .skip}

- Objective-C Runtime: You'll write Rust code that uses the Objective-C runtime to interact with Objective-C classes and objects.
- Messaging: You'll send messages to Objective-C objects.
- Data Marshaling: You'll need to convert data between Rust types and Objective-C types (e.g., converting Rust [strings][p~strings] to NSStrings, and vice-versa).
- Memory Management: Objective-C uses reference counting. You'll need to be mindful of memory management to avoid leaks or crashes.

Keep strong ownership in `Retained<T>` and use scoped autorelease pools where APIs create autoreleased objects. Obey each framework's main-thread requirements; UI APIs often require a `MainThreadMarker`. Raw message sends require the correct selector, argument types, ownership convention, and receiver state. Restrict `unsafe` to operations whose contracts you can establish.

### Implement an Objective-C Class {#objc-classes}

Use [`define_class!`][ffi~objc-class] for a delegate or another Rust-implemented Objective-C class. Declare its superclass, choose its thread kind, and store Rust state in an ivars type. Implement the required protocol methods with the correct selectors and signatures. The macro checks class definitions, but its unsafe superclass and protocol contracts still require review. Do not pass a Rust closure where an Objective-C block or delegate is expected without a bridge that owns it for the required lifetime.

### Expose Rust to Swift {#objc-swift}

[`swift-bridge`][ffi~swift-bridge] generates Rust/Swift glue for a declared shared interface. [UniFFI](generate_ffi_bindings.md#uniffi) generates Swift bindings when you want to share a Rust API with several languages. Neither automatically makes every Rust type or Apple framework object transferable across the boundary. Keep errors, cancellation, callbacks, and thread affinity explicit, then link the produced library into the Xcode application. See the [Rust mobile platform projects][ffi~mobile] for application-level choices.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
