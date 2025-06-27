#![allow(dead_code)]
// // ANCHOR: example
// #![allow(unused)]
// use rustler::Env;
// use rustler::NifResult;

// // The `rustler` crate provides macros and functions for creating NIFs
// // (Native Implemented Functions). NIFs are functions written in Rust that
// // can be called directly from Erlang or Elixir code. This allows you to
// // leverage the performance and safety of Rust within your Erlang/Elixir
// // applications.

// // `greet` is a NIF that takes a string and returns a string.
// // `Env` represents the current execution environment in Elixir.
// #[rustler::nif]
// fn greet(env: Env, name: String) -> NifResult<String> {
//     Ok(format!("Hello, {name}!"))
// }

// // `add` is a NIF that takes two integers and returns their sum.
// #[rustler::nif]
// fn add(env: Env, a: i32, b: i32) -> NifResult<i32> {
//     Ok(a + b)
// }

// // Initialize the NIF module.
// // "Elixir.MyRustLibrary" specifies the name of the Elixir module that will
// // call these NIFs.
// rustler::init!("Elixir.MyRustLibrary");
// // ANCHOR_END: example

// // [review; how to test](https://github.com/john-cd/rust_howto/issues/1080)
