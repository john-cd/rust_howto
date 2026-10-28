<!-- TODO Reviewed ??-??-2026 -->

# Generate FFI Bindings to Java Code

{{#include java.incl.md}}

Interfacing with Java from Rust is typically done using the Java Native Interface (JNI).

## `jni` {#jni}

[![jni][c~jni~docs~badge]][c~jni~docs] [![jni~crates.io][c~jni~crates.io~badge]][c~jni~crates.io] [![jni~repo][c~jni~repo~badge]][c~jni~repo] [![jni~lib.rs][c~jni~lib.rs~badge]][c~jni~lib.rs]{{hi:jni}}{{hi:Ffi}}{{hi:Java}}{{hi:jni}} [![cat~api-bindings][cat~api-bindings~badge]][cat~api-bindings]{{hi:API bindings}}

[`jni`][c~jni~docs]↗{{hi:jni}} provides Rust bindings to the `Java` JNI API.

Declare a Java native method and load its library. The package and class must match the exported Rust symbol; the checked-in Java caller is:

```java
package com.example;

public class NativeLibrary {
    private static native String greet(String name);

    static {
        System.loadLibrary("mylib");
    }

    public static void main(String[] args) {
        System.out.println(greet("World"));
    }
}
```

The [JNI crate guide][ffi~jni-guide] uses `extern "system"` for native entry points. With Rust 2024, export the symbol with `#[unsafe(no_mangle)]`. `Java_com_example_NativeLibrary_greet` is the name for this non-overloaded method. Generate a header with `javac -h` to check signatures and escaping when names are more complex.

The JNI Rust API is available on Windows. From `later/crates`, compile this example with the Java feature on Windows or another desktop target:

```console
cargo test --locked -p development_tools_ffi --example java --features java
```

Compiling the Rust JNI code does not require a JDK because the JVM supplies
`JNIEnv` when it calls the native function. Running the Java program requires a
JDK with `java` and `javac` on `PATH`; set `JAVA_HOME` to the JDK root for build
tools that require it. The devcontainer and Windows dependency script install
Temurin/OpenJDK 21. The caller also needs a separately built Rust `cdylib`;
Windows loads the resulting `.dll` through `System.loadLibrary`.

The example targets `jni` 0.22: its `EnvUnowned` wraps the incoming environment, and `with_env` supplies a checked `Env` for the operation. Older tutorials using `JNIEnv` refer to a different API generation. The example converts a `JString` with `try_to_string`, creates a Java string with `new_string`, and resolves failures through `ThrowRuntimeExAndDefault`, rather than unwinding into Java.

```rust,editable
{{#include ../../../crates/cats/development_tools_ffi/examples/java/jni.rs:example}}
```

## Build Tools {#build-tools}

- [`cargo`][c~cargo~docs]↗{{hi:cargo}}: For building the Rust library.
- Maven or Gradle: For building the Java side and integrating with the Rust library.

To run the caller, create a standalone library crate named `mylib` with `jni = "0.22.4"`, `[lib] crate-type = ["cdylib"]`, and the included Rust code as `src/lib.rs`. Copy `NativeLibrary.java` into that project's root, then run:

```bash
cargo build
javac -h . -d . NativeLibrary.java
java -Djava.library.path=target/debug com.example.NativeLibrary
```

The expected output is `Hello, World!`. `-d .` creates the package directories. Adjust the library path if Cargo uses an external target directory. The book's `java` example is a binary compile/test target; it does not produce this JNI library automatically. Run the Java smoke test separately from the pure-Rust greeting tests.

## Key Concepts and Workflow {#key-concepts-and-workflow}

- JNI: You'll write Rust code that interacts with the JNI API to call Java methods, access Java fields, and create Java objects.
- Native Methods: You'll define native methods in Java that are implemented in Rust.
- Data Marshaling: You'll need to convert data between Rust and Java types (e.g., converting Rust strings to Java strings, and vice-versa).
- Exception Handling: You'll need to handle Java exceptions in your Rust code.

The [`jni`][c~jni~docs]↗{{hi:jni}} crate is the core crate for JNI interaction. Code generation is often a necessary step, and the exact approach can vary.

### Exchange Strings, Arrays, and References {#jni-values}

Java strings have their own representation; do not cast a Rust UTF-8 pointer to `jstring`. Use `JString` conversions. For primitive arrays, use the crate's typed array wrappers and region/copy APIs, validate lengths, and release any borrowed elements according to their guard's contract. Object references are normally local to a JNI frame. Promote retained objects to global references, and never move a thread's JNI environment to another thread. Propagate pending Java exceptions before continuing JNI calls.

### Call Java from Rust {#call-java}

To launch a JVM from a Rust executable, enable the `jni` crate's `invocation` feature, build VM arguments with `InitArgsBuilder`, create a `JavaVM`, and attach the calling thread. The environment's `call_method` invokes instance methods using JNI descriptors, such as `()I` for a method returning `int`. Check both the returned `Result` and Java exception state. This route needs an installed, discoverable JVM; see [Launching JVM from Rust][ffi~jni-invocation]. Android applications generally receive an existing VM from the platform instead of launching one.

### Choose Binding Generation or a C ABI {#java-binding-options}

For a larger API, binding generators reduce handwritten JNI declarations. [UniFFI](generate_ffi_bindings.md#uniffi) has a supported-language list: Kotlin and Java are separate targets, so do not assume Kotlin output is a Java binding. Check a Java/JNI generator's supported Rust types and release workflow before adopting it.

On Java 22 and later, the [Foreign Function & Memory API][ffi~java-ffm] can call an exported C ABI from a Rust `cdylib` without JNI. Use `extern "C"`, C-compatible layouts, explicit buffer ownership, and matching `FunctionDescriptor` definitions; arenas govern the lifetime of foreign memory. This does not expose arbitrary Rust types automatically. For Android packaging and Kotlin/Swift consumers, see the [Rust mobile platform projects][ffi~mobile] and the [general FFI guide](generate_ffi_bindings.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
