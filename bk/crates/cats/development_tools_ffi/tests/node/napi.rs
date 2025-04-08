// // ANCHOR: example
// // Framework for building pre-compiled Node.js addons in Rust.

// /// Import the preludes:
// use napi::bindgen_prelude::*;
// use napi_derive::napi;
// use napi::Env;
// use napi::Error;
// use napi::JsFunction;
// use napi::JsObject;
// use napi::JsString;
// use napi::Result;

// #[napi] // Indicates that the function is exposed to JavaScript.
// fn hello(mut this: JsObject, args: Vec<napi::Value>) -> Result<JsString> {
//     let env = this.env();
//     let name = args.get(0)?.as_string()?.into_owned();

//     let greeting = format!("Hello, {}!", name);
//     env.create_string(&greeting)
// }

// #[napi]
// fn create_function(
//     mut this: JsObject,
//     args: Vec<napi::Value>,
// ) -> Result<JsFunction> {
//     let env = this.env();

//     let js_fn = JsFunction::new(&env, hello)?;
//     this.set_named_property("hello", &js_fn)?;

//     Ok(js_fn)
// }

// #[napi]
// fn init(mut exports: JsObject) -> Result<()> {
//     exports.set_named_property("createFunction", &create_function)?;
//     Ok(())
// }

// napi_module_init!(init);

// // ANCHOR_END: example

// [finish; https://lib.rs/crates/napi; https://github.com/napi-rs/package-template](https://github.com/john-cd/rust_howto/issues/1032)
