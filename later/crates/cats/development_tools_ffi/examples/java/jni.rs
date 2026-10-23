#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Calling Rust from Java with the Java Native Interface.

use jni::EnvUnowned;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::JClass;
use jni::objects::JString;

// In `Cargo.toml`, add jni = "0.22.4" and build the library as a cdylib.
// Build the Rust code as a dynamic library (e.g., a .so file for Linux, .dll
// for Windows): add a new `[lib]` section and under it,
// `crate_type = ["cdylib"]`
// if you run `cargo build` from inside the crate directory, you
// should see a `mylib.dll` on Windows, `libmylib.so` on Linux, or
// `libmylib.dylib` on macOS in the `target/debug` directory.
// Compile the java code with the javac command:
// ```sh
// javac NativeLibrary.java
// ```
// Generate a C header declaration with `javac -h . NativeLibrary.java`.
// Run Java with `java -Djava.library.path=target/debug
// com.example.NativeLibrary` so the JVM can find `mylib.dll`, `libmylib.so`, or
// `libmylib.dylib`.

/// Returns a greeting for `name`; called from Java as `NativeLibrary.greet`.
// `no_mangle` preserves the symbol name expected by the JVM.
#[unsafe(no_mangle)]
// The Java Native Interface (JNI) function name follows the convention:
// Java_<package_name>_<class_name>_<method_name>.
// unowned_env is the FFI-safe JNI environment passed by the JVM. JClass points
// to the Java class that owns this native method and is unused in this example.
pub extern "system" fn Java_com_example_NativeLibrary_greet<'a>(
    mut unowned_env: EnvUnowned<'a>,
    _: JClass<'a>,
    name: JString<'a>,
) -> JString<'a> {
    unowned_env
        .with_env(|env| -> jni::errors::Result<_> {
            let name_str = name.try_to_string(env)?;
            let hello = format!("Hello, {name_str}!");
            env.new_string(&hello)
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}
// ANCHOR_END: example
// [finish](https://github.com/john-cd/rust_howto/issues/1029)
