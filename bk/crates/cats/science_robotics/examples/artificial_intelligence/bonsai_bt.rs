#![allow(dead_code)]
// ANCHOR: example
// COMING SOON
// ANCHOR_END: example
//! # Behavior Tree Example
//!
//! This example demonstrates the use of a Behavior Tree (BT) to control a
//! simple counter. The BT consists of a sequence of actions that increment or
//! decrement the counter, with pauses in between.
//!
//! A Behavior Tree (BT) is a data structure in which we can set the rules of
//! how certain behaviors can occur, and the order in which they would
//! execute. BTs are a very efficient way of creating complex systems that are
//! both modular and reactive. These properties have led to the spread of BT
//! from computer game programming to AI and Robotics.

use std::collections::HashMap;

// An Action can either be "condition" which does not alter the system and
// returns either Success or Failure.
// Or it can be an "act" that can alter the system and returns either
// Success, Failure or Running.
use bonsai_bt::Action;
// The execution state of a behavior tree, along with a "blackboard" (state
// shared between all nodes in the tree).
use bonsai_bt::BT;
// An Event.
use bonsai_bt::Event;
// A sequence runs behaviors one by one.
use bonsai_bt::Sequence;
// The result of a behavior or action.
use bonsai_bt::Status;
// The behavior or action succeeded.
use bonsai_bt::Success;
// Update arguments, such as delta time in seconds.
// To move the behavior tree forward in time it must be ticked on each
// iteration of the game/application loop.
use bonsai_bt::UpdateArgs;
// Waits an amount of time before continuing.
use bonsai_bt::Wait;

#[derive(Clone, Debug, Copy)]
pub enum Actions {
    // Increment accumulator.
    Inc,
    // Decrement accumulator.
    Dec,
}

/// This function sets up a behavior tree, runs it in a loop, and prints the
/// result.
fn main() -> anyhow::Result<()> {
    use Actions::Dec;
    use Actions::Inc;

    // Create a `HashMap` to store the state of the counter.
    let mut blackboard = HashMap::new();
    blackboard.insert("counter".to_string(), 0);

    // Create a composite behavior.
    // Runs behaviors one by one until all succeeded.
    let behavior = Sequence(vec![
        Wait(0.1),
        Action(Inc),
        Wait(0.2),
        Action(Inc),
        Wait(0.3),
        Action(Dec),
    ]);
    // You may use: `Sequence`, `Select`, `If`, `Invert`, `While`, `WhenAll`,
    // `WhenAny`.

    // A Behavior Tree forms a tree structure, where each node represents a
    // process. When the process terminates, it signals `Success` or `Failure`.
    let mut bt = BT::new(behavior, blackboard);

    let dt: f64 = 0.1f64;

    // Run the behavior tree.
    loop {
        let e: Event = UpdateArgs { dt }.into();
        let (status, _dt) = bt.tick(&e, &mut |args, blackboard| {
            let acc = blackboard.get_mut("counter").unwrap();
            match *args.action {
                Actions::Inc => {
                    *acc += 1;
                    println!("Inc");
                    (Success, args.dt)
                }
                Actions::Dec => {
                    *acc -= 1;
                    println!("Dec");
                    (Success, args.dt)
                }
            }

        }).ok_or(anyhow::anyhow!("Attempted to tick after this tree has already returned Status::Success or Status::Failure."))?;

        match status {
            Status::Success => {
                println!("Behavior tree completed successfully!");
                break;
            }
            Status::Running => {
                // Wait for the dt duration before the next tick, if desired
            }
            Status::Failure => {
                break;
            }
        }
    }

    let count = bt.blackboard().get("counter").unwrap();
    assert_eq!(*count, 1);

    // If the behavior tree concludes (reaches a steady state),
    // you can reset the tree back to it's initial state at t = 0.0.
    bt.reset_bt();

    Ok(())
}

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// [review / exapnd](https://github.com/john-cd/rust_howto/issues/841)
// https://github.com/Sollimann/bonsai/blob/1aa74afcb11603e86d5c7e941a70b2533e844e16/examples/src/async_drone/main.rs
// https://github.com/Sollimann/bonsai/blob/main/docs/concepts/README.md
