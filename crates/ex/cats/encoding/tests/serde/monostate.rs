// // ANCHOR: example
// // ANCHOR_END: example

// use monostate::MonoState;
// use std::sync::Arc;

// monostate is a crate in Rust that allows you to create a singleton type which
// is always the same value and thus can be used for types that need to be Send
// and Sync without any synchronization.

// // Define a struct with MonoState
// #[derive(Debug, Default)]
// struct MySingleton {
//     state: MonoState,
//     data: Arc<String>,
// }

// fn main() {
//     let instance1 = MySingleton {
//         state: MonoState::default(),
//         data: Arc::new("Hello, MonoState!".to_string()),
//     };

//     let instance2 = instance1.clone();

//     println!("Instance 1: {:?}", instance1);
//     println!("Instance 2: {:?}", instance2);
// }

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/754)
