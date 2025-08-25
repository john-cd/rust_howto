#![allow(dead_code)]
// // ANCHOR: example
// #![allow(unused_imports)]
// //! This example demonstrates how to create a native library in Rust that can
// be //! called from Java using JNI.

// // This is the interface to the JVM that we'll call the majority of our
// // methods on.
// use jni::JNIEnv;
// // The following objects are what we should use as arguments to your
// // native function. They carry extra lifetime information to prevent them
// // escaping this context and getting used after being GC'd.
// use jni::objects::{JClass, JString};

// // This is just a pointer. We'll be returning it from our function. We
// // can't return one of the objects with lifetime information because the
// // lifetime checker won't let us.
// // use jni::sys::jstring;

// // In `Cargo.toml`, under [dependencies], add jni = "0.21.1"
// // Build the Rust code as a dynamic library (e.g., a .so file for Linux, .dll
// // for Windows): add a new `[lib]` section and under it,
// // `crate_type = ["cdylib"]`
// // if you run `cargo build` from inside the crate directory, you
// // should see a `libmylib.so` (if you're on Linux) or a `libmylib.dylib` (if
// // we are on OSX) in the `target/debug` directory.
// // Compile the java code with the javac command:
// // ```sh
// // javac NativeLibrary.java
// // ```
// // Get a `NativeLibrary.h` output to your directory with the following
// command: // ```
// // javac - h.NativeLibrary.java
// // ```
// // Linking: on Linux, we can export
// // ```
// // LD_LIBRARY_PATH=/path/to/mylib/target/debug.
// // ```
// //
// // This example defines a single native function `greet` that takes a Java
// // String as input, formats a greeting message, and returns a new Java
// String.

// // `no_mangle` prevents the Rust compiler from mangling the function name,
// // ensuring that Java can correctly find and call it.
// #[unsafe(no_mangle)]
// // The Java Native Interface (JNI) function name follows the convention:
// // Java_<package_name>_<class_name>_<method_name>.
// // env: JNIEnv is a pointer to the JNI environment, which provides access to
// // JNI functions. JClass is a pointer to the Java class that this native
// // method belongs to. It's often unused.
// pub extern "C" fn Java_com_example_NativeLibrary_greet<'a>(
//     mut env: JNIEnv<'a>,
//     _: JClass<'a>,
//     name: JString<'a>,
// ) -> JString<'a> {
//     // Converts the JString argument into a Rust String. The `get_string`
//     // method returns a `Result` that needs to be handled.
//     let name_str = env.get_string(&name).expect("Invalid UTF-8");
//     // Formats a greeting message using the input name.
//     let hello = format!("Hello, {:?}!", name_str);

//     // Creates a new JString object from the Rust String.
//     env.new_string(&hello)
//         .expect("Couldn't create Java String!")
// }
// // ANCHOR_END: example

// // #[test]
// // fn test() {
// // }
// // [finish](https://github.com/john-cd/rust_howto/issues/1029)
