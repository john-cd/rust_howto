// // ANCHOR: example
// use crux_core::App;
// use serde::Deserialize;
// use serde::Serialize;

// // Crux is a framework for writing Rust applications that can target both
// // native and web platforms using a single codebase.
// // Crux helps you share your app's business logic and behavior across mobile
// // (iOS/Android) and web - as a single reusable core built with Rust.

// // Define a Counter struct that holds the state of the application.
// #[derive(Crux, Default)]
// struct Counter {
//     count: i32,
// }

// // Define a Msg enum for the different messages the application can handle.
// #[derive(Serialize, Deserialize)]
// enum Msg {
//     Increment,
//     Decrement,
// }

// // Implement the App trait for the Counter struct to define how the state
// // should be updated in response to messages.
// impl App for Counter {
//     type Capabilities = ();
//     type Event = Msg;
//     type Model = ();

//     fn update(&mut self, msg: Msg) -> Vec<()> {
//         match msg {
//             Msg::Increment => self.count += 1,
//             Msg::Decrement => self.count -= 1,
//         }
//         vec![]
//     }

//     fn view(&self, _: &<Self as App>::Model) -> <Self as App>::ViewModel {}
// }

// fn main() {
//     let mut app = Counter::default();

//     // Simulate user interactions
//     app.update(Msg::Increment);
//     app.update(Msg::Increment);
//     app.update(Msg::Decrement);

//     println!("Count: {}", app.count);
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/880)
