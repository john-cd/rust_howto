#![allow(dead_code)]
// // ANCHOR: example
// //! This example demonstrates a basic actor implementation.
// //!
// //! An actor is a concurrent entity that can receive and process messages.
// //! It's a fundamental building block for concurrent systems.
// use std::time::Duration;
// use actors::{Actor, Context, SystemBuilder, Sender};

// struct MyActor;

// impl Actor for MyActor {
//     type Msg = String;

//     fn recv(&mut self, ctx: &Context<String>, msg: String, sender: Sender) {
//         println!("received {}", msg);
//     }
// }

// #[test]
// fn test() {
//     let sys = SystemBuilder::new().name("my-app").create().unwrap();
//     // Every actor has a name that is required to be unique among
//     // its singlings (those actors sharing the same parent actor).
//     let my_actor = sys.actor_of::<MyActor>("my-actor").unwrap();
//     my_actor.tell("Hello!".to_string(), None);

//     // force main to wait before exiting program
//     std::thread::sleep(Duration::from_millis(500));
// }
// // ANCHOR_END: example
// // [finish](https://github.com/john-cd/rust_howto/issues/1011)
