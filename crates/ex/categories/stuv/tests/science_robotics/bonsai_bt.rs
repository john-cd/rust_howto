// // ANCHOR: example
// use std::collections::HashMap;

// use bonsai_bt::Action;
// use bonsai_bt::BT;
// use bonsai_bt::Event;
// use bonsai_bt::Sequence;
// use bonsai_bt::Status;
// use bonsai_bt::Success;
// use bonsai_bt::UpdateArgs;
// use bonsai_bt::Wait;

// use std::time::Duration;

// // Use the Bonsai Behavior Tree (bonsai-bt) library in Rust.

// #[derive(Clone, Debug, Copy)]
// pub enum Actions {
//     ///! Increment accumulator.
//     Inc,
//     ///! Decrement accumulator.
//     Dec,
// }

// fn main() {
//     // Define a simple action that increments a counter
//     let increment_action = |_args: &mut UpdateArgs| -> Result<Event, String>
// {         println!("Incrementing...");
//         Ok(Event::Running)
//     };

//     // Define a simple action that decrements a counter
//     let decrement_action = |_args: &mut UpdateArgs| -> Result<Event, String>
// {         println!("Decrementing...");
//         Ok(Event::Success(Success::new()))
//     };

//     // Create a HashMap to store the state of the counter
//     let mut blackboard = HashMap::new();
//     blackboard.insert("counter".to_string(), 0);

//     // Create a simple behavior tree
//     let tree = BT::builder()
//         .sequence()
//         .push(increment_action)
//         .push(increment_action)
//         .push(decrement_action)
//         .build();

//     // Run the behavior tree
//     let mut tree = tree.unwrap();
//     loop {
//         match tree.tick(&mut blackboard, &mut UpdateArgs::default()) {
//             Ok(Event::Success(_)) => {
//                 println!("Behavior tree completed successfully!");
//                 break;
//             }
//             Ok(Event::Running) => {
//                 // Wait for a short duration before the next tick
//                 std::thread::sleep(Duration::from_millis(500));
//             }
//             Err(e) => {
//                 println!("Error: {}", e);
//                 break;
//             }
//         }
//     }
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/841)
