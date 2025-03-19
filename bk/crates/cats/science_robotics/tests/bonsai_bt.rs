// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use std::collections::HashMap;
// use std::time::Duration;

// // An Action can either be "condition" which does not alter the system and
// returns either Success or Failure // Or it can be an "act" that can alter the
// system and returns either Success, Failure or Running use bonsai_bt::Action;
// // The execution state of a behavior tree, along with a "blackboard" (state
// shared between all nodes in the tree). use bonsai_bt::BT;
// // An Event
// use bonsai_bt::Event;
// // A sequence runs behaviors one by one
// use bonsai_bt::Sequence;
// // The result of a behavior or action
// use bonsai_bt::Status;
// // The behavior or action succeeded.
// use bonsai_bt::Success;
// // Update arguments, such as delta time in seconds.
// // To move the behavior tree forward in time it must be ticked on each
// iteration of the game/application loop use bonsai_bt::UpdateArgs;
// // Waits an amount of time before continuing
// use bonsai_bt::Wait;

// // A Behavior Tree (BT) is a data structure in which we can set the rules of
// how // certain behavior's can occur, and the order in which they would
// execute. // BTs are a very efficient way of creating complex systems that are
// both // modular and reactive. These properties have led to the spread of BT
// from // computer game programming to AI and Robotics.

// #[derive(Clone, Debug, Copy)]
// pub enum Actions {
//     // Increment accumulator.
//     Inc,
//     // Decrement accumulator.
//     Dec,
// }

// fn main() -> anyhow::Result<()> {
//     use crate::Actions::{Inc, Dec};

//     // Create a HashMap to store the state of the counter
//     let mut blackboard = HashMap::new();
//     blackboard.insert("counter".to_string(), 0);

//     // Create a composite behavior.
//     // Runs behaviors one by one until all succeeded.
//     let behavior = Sequence(vec![
//         Wait(0.1),
//         Action(Inc),
//         Wait(0.2),
//         Action(Inc),
//         Wait(0.3),
//         Action(Dec),
//     ]);
//     // You may use: Sequence, Select, If, Invert, While, WhenAll, WhenAny

//     // A Behavior Tree forms a tree structure where each node represents a
//     // process. When the process terminates, it signals Success or Failure.
//     let mut bt = BT::new(behavior, blackboard);

//     let dt: f64 = 0f64;

//     // Run the behavior tree
//     loop {
//         let e: Event = UpdateArgs { dt }.into();
//         let (status, _dt) = bt.tick(&e, &mut |args, blackboard| match
// *args.action {             Actions::Inc => {
//                 acc += 1;
//                 (Success, args.dt)
//             }
//             Actions::Dec => {
//                 acc -= 1;
//                 (Success, args.dt)
//             }
//         })?;

//         // update counter in blackboard
//         let bb = bt.get_blackboard();

//         bb.entry("count".to_string())
//             .and_modify(|count| *count = acc)
//             .or_insert(0)
//             .to_owned();

//         match status {
//             Success => {
//                 println!("Behavior tree completed successfully!");
//                 break;
//             }
//             Running => {
//                 // Wait for a short duration before the next tick
//                 std::thread::sleep(Duration::from_millis(50));
//             }
//             Failure => {
//                 println!("Error: {}", e);
//                 break;
//             }
//         }
//     }

//     // if the behavior tree concludes (reaches a steady state)
//     // you can reset the tree back to it's initial state at t=0.0
//     bt.reset_bt();

//     Ok(())
// }

// #[test]
// fn test() {
//     main();
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/841)
// // https://github.com/Sollimann/bonsai/blob/1aa74afcb11603e86d5c7e941a70b2533e844e16/examples/src/async_drone/main.rs
// // https://github.com/Sollimann/bonsai/blob/main/docs/concepts/README.md
