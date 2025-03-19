// ANCHOR: example
#![allow(unused)]
use rustler::Env;
use rustler::NifResult;

// `rustler` provides macros and functions for creating NIFs
// (Native Implemented Functions).

// `greet` is a NIF that takes a string and returns a string.
// `Env` represents the current execution environment in Elixir.
#[rustler::nif]
fn greet(env: Env, name: String) -> NifResult<String> {
    Ok(format!("Hello, {}!", name))
}

#[rustler::nif]
fn add(env: Env, a: i32, b: i32) -> NifResult<i32> {
    Ok(a + b)
}

// Initialize the NIF module
// "Elixir.MyRustLibrary" specifies the name of the Elixir module that will call
// these NIFs
rustler::init!("Elixir.MyRustLibrary");
// ANCHOR_END: example

// [WIP review; how to test](https://github.com/john-cd/rust_howto/issues/1080)
