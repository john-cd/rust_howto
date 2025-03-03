# Generate FFI bindings to Java code

{{#include java.incl.md}}

## `jni` {#jni}

[![jni][c-jni-badge]][c-jni] [![jni-crates.io][c-jni-crates.io-badge]][c-jni-crates.io] [![jni-github][c-jni-github-badge]][c-jni-github] [![jni-lib.rs][c-jni-lib.rs-badge]][c-jni-lib.rs]{{hi:jni}}{{hi:Ffi}}{{hi:Java}}{{hi:jni}} [![cat-api-bindings][cat-api-bindings-badge]][cat-api-bindings]{{hi:API bindings}}

[`jni`][c-jni]⮳{{hi:jni}} provides Rust bindings to the `Java` JNI API.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/tests/java/jni.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1072)

Interfacing with Java from Rust is typically done using the Java Native Interface (JNI). Here's a breakdown:

## JNI Bindings

jni: This is the primary crate for working with JNI in Rust. It provides bindings to the JNI API and helps with managing Java objects, calling Java methods, and handling exceptions.
Code Generation (Often Needed):

You'll often need to generate some boilerplate code to bridge between Rust and Java. There isn't one single tool that does this perfectly, and it can be a manual process or involve custom scripting. jni-rs (mentioned below) can help.)

## jni-rs (Helpful Tool)

While not a binding generator in the traditional sense, jni-rs provides helpful [macros][p-macros] and utilities to make writing JNI code in Rust easier. It simplifies many common JNI tasks.

## Build Tools

cargo: (For building the Rust library.)
Maven or Gradle: (For building the Java side and integrating with the Rust library.)

## Key Concepts and Workflow

JNI: You'll write Rust code that interacts with the JNI API to call Java methods, access Java fields, and create Java objects.
Native Methods: You'll define native methods in Java that are implemented in Rust.
Data Marshaling: You'll need to convert data between Rust and Java types (e.g., converting Rust strings to Java strings, and vice-versa).
Class Loading: You'll need to ensure that the Java classes your Rust code interacts with are loaded correctly.
Exception Handling: You'll need to handle Java exceptions in your Rust code.

The `jni` crate is the core crate for JNI interaction. jni-rs is highly recommended to simplify development. Code generation is often a necessary step, and the exact approach can vary. Be prepared for some manual work and boilerplate code when working with JNI.

</div>
