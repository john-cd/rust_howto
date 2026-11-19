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

/// Formats the greeting independently of the JVM.
///
/// # Arguments
///
/// * `input` - Name to include in the greeting.
pub fn hello(input: &str) -> String {
    format!("Hello, {input}!")
}

/// Returns a greeting for `input`; called from Java as `HelloWorld.hello`.
// `no_mangle` preserves the symbol name expected by the JVM.
#[unsafe(no_mangle)]
// The Java Native Interface (JNI) function name follows the convention:
// Java_<package>_<class>_<method> for this non-overloaded method.
// `unowned_env` is the FFI-safe JNI environment passed by the JVM. JClass
// points to the Java class that owns this native method and is unused in this
// example.
pub extern "system" fn Java_com_example_HelloWorld_hello<'a>(
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

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Passes the Java caller’s name through the greeting response.
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
