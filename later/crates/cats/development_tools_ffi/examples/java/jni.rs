// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calling Rust from Java with the Java Native Interface.
//!
//! JNI (Java Native Interface) is how Java calls native code. A Rust function
//! is exported with a name derived from its Java class and method, and receives
//! a handle to the JVM. It lets you reuse Rust libraries from Java and Android
//! apps.

use jni::EnvUnowned;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::JClass;
use jni::objects::JString;
use jni::sys::jstring;

// In `Cargo.toml`, add jni = "0.22.4" and build the library as a cdylib.
// Build the Rust code as a dynamic library (e.g., a .so file for Linux, .dll
// for Windows): add a new `[lib]` section and under it,
// `crate_type = ["cdylib"]`
// if you run `cargo build` from inside the crate directory, you
// should see a `mylib.dll` on Windows, `libmylib.so` on Linux, or
// `libmylib.dylib` on macOS in the `target/debug` directory.
// Compile the java code with the javac command:
// ```sh
// javac HelloWorld.java
// ```
// Generate a C header declaration with `javac -h . HelloWorld.java`.
// Run Java with `java -Djava.library.path=target/debug HelloWorld`
// so the JVM can find `mylib.dll`, `libmylib.so`, or `libmylib.dylib`.

/// Formats the greeting independently of the JVM.
pub fn hello(input: &str) -> String {
    format!("Hello, {input}!")
}

/// Returns a greeting for `input`; called from Java as `HelloWorld.hello`.
// `no_mangle` preserves the symbol name expected by the JVM.
#[unsafe(no_mangle)]
// The Java Native Interface (JNI) function name follows the convention:
// Java_<class_name>_<method_name> for default packages.
// `unowned_env` is the FFI-safe JNI environment passed by the JVM. JClass
// points to the Java class that owns this native method and is unused in this
// example.
pub extern "system" fn Java_HelloWorld_hello<'a>(
    mut unowned_env: EnvUnowned<'a>,
    _: JClass<'a>,
    input: JString<'a>,
) -> jstring {
    unowned_env
        .with_env(|env| -> jni::errors::Result<_> {
            let input_str = input.try_to_string(env)?;
            let greeting = hello(&input_str);
            let java_str = env.new_string(&greeting)?;
            Ok(java_str.into_raw())
        })
        .resolve::<ThrowRuntimeExAndDefault>()
}
// ANCHOR_END: example

/// Runs the JNI example logic without starting a JVM.
pub fn run() {
    let msg = hello("World");
    println!("{msg}");
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks the text returned by the native greeting function without
    /// starting a JVM.
    #[test]
    fn hello_preserves_the_java_caller_name() {
        assert_eq!(hello("World"), "Hello, World!");
    }

    /// Verifies the `run()` entry point function.
    #[test]
    fn test_jni_example_entry_point() {
        run();
    }
}
