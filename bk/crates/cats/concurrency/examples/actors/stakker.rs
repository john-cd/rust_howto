#![allow(dead_code)]
// // ANCHOR: example
// use stakker::*;

// // Stakker is a lightweight, low-level, single-threaded actor runtime.
// // It is designed to be layered on top of whatever event loop the user
// // prefers to use. Use it to build concurrent, distributed applications.

// // Define the message type that actors will exchange.
// enum MyMessage {
//     Increment(u32),
//     GetValue,
//     PrintValue,
// }

// // Define the actor's state and behavior.
// struct Counter {
//     count: u32,
//     sender: Sender<MyMessage>, // To send messages back (if needed)
// }

// impl Counter {
//     fn new(sender: Sender<MyMessage>) -> Self {
//         Counter { count: 0, sender }
//     }

//     fn handle_message(&mut self, message: MyMessage) {
//         match message {
//             MyMessage::Increment(amount) => {
//                 self.count += amount;
//             }
//             MyMessage::GetValue => {
//                 println!("Current value: {}", self.count);
//                 // Example of sending a message back
//                 // (if another actor is listening)
//                 // self.sender.send(MyMessage::PrintValue);
//                 // If you had a receiver
//                 }
//             MyMessage::PrintValue => {
//                 println!("Value to print: {}", self.count);
//             }
//         }
//     }
// }

// fn main() {
//     // Create a Stakker system.
//     let mut system = Stakker::new();

//     // Create a new actor.
//     let (counter_sender, counter_handle) = system.spawn(
//         // The actor's initial state.
//         Counter::new(counter_sender.clone()),  // Clone needed if actor sends
// back                  // The actor's message handler.
//         |counter, message| counter.handle_message(message),
//     );

//     // Send messages to the actor.
//     counter_sender.send(MyMessage::Increment(5));
//     counter_sender.send(MyMessage::Increment(10));
//     counter_sender.send(MyMessage::GetValue);
//     counter_sender.send(MyMessage::Increment(2));
//     counter_sender.send(MyMessage::GetValue);

//     // Run the Stakker system.  This will process the messages.
//     system.run(());
//     // The empty tuple () is the "external input" in this case

//     // If you need to wait for the actor to finish
//     // system.wait(counter_handle);
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/94)
