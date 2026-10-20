# Generate FFI Bindings to Java Code

{{#include java.incl.md}}

Interfacing with Java from Rust is typically done using the Java Native Interface (JNI).

## `jni` {#jni}

[![jni][c~jni~docs~badge]][c~jni~docs] [![jni~crates.io][c~jni~crates.io~badge]][c~jni~crates.io] [![jni~repo][c~jni~repo~badge]][c~jni~repo] [![jni~lib.rs][c~jni~lib.rs~badge]][c~jni~lib.rs]{{hi:jni}}{{hi:Ffi}}{{hi:Java}}{{hi:jni}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}

[`jni`][c~jni~docs]↗{{hi:jni}} provides Rust bindings to the `Java` JNI API.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/java/jni.rs:example}}
```

## Build Tools {#build-tools}

- [`cargo`][c~cargo~docs]↗{{hi:cargo}}: For building the Rust library.
- Maven or Gradle: For building the Java side and integrating with the Rust library.

## Key Concepts and Workflow {#key-concepts-and-workflow}

- JNI: You'll write Rust code that interacts with the JNI API to call Java methods, access Java fields, and create Java objects.
- Native Methods: You'll define native methods in Java that are implemented in Rust.
- Data Marshaling: You'll need to convert data between Rust and Java types (e.g., converting Rust strings to Java strings, and vice-versa).
- Exception Handling: You'll need to handle Java exceptions in your Rust code.

The [`jni`][c~jni~docs]↗{{hi:jni}} crate is the core crate for JNI interaction. Code generation is often a necessary step, and the exact approach can vary.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write](https://github.com/john-cd/rust_howto/issues/1072)
</div>
