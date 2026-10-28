#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! # Behavior Tree Example
//!
//! This example demonstrates the use of a Behavior Tree (BT) to control a
//! simple counter. The BT consists of a sequence of actions that increment or
//! decrement the counter, with pauses in between.

use std::collections::HashMap;

use bonsai_bt::Action;
use bonsai_bt::BT;
use bonsai_bt::Event;
use bonsai_bt::Sequence;
use bonsai_bt::Status;
use bonsai_bt::Success;
use bonsai_bt::UpdateArgs;
use bonsai_bt::Wait;

/// The actions of the behavior tree.
#[derive(Clone, Debug, Copy)]
pub enum Actions {
    /// Increments the accumulator.
    Inc,
    /// Decrements the accumulator.
    Dec,
}

/// Processes an action on the accumulator and returns status.
fn process_action(action: Actions, acc: &mut i32) -> Status {
    match action {
        Actions::Inc => {
            *acc += 1;
            println!("Inc");
        }
        Actions::Dec => {
            *acc -= 1;
            println!("Dec");
        }
    }
    Success
}

/// This function sets up a behavior tree, runs it in a loop, and prints the
/// result.
fn main() -> anyhow::Result<()> {
    use Actions::Dec;
    use Actions::Inc;

    let mut blackboard = HashMap::new();
    blackboard.insert("counter".to_owned(), 0);

    let behavior = Sequence(vec![
        Wait(0.1),
        Action(Inc),
        Wait(0.2),
        Action(Inc),
        Wait(0.3),
        Action(Dec),
    ]);

    let mut bt = BT::new(behavior, blackboard);
    let dt = 0.1;

    loop {
        let e: Event = UpdateArgs { dt }.into();
        let (status, _dt) = bt.tick(&e, &mut |args, blackboard| {
            let Some(acc) = blackboard.get_mut("counter") else {
                return (Status::Failure, args.dt);
            };
            let s = process_action(*args.action, acc);
            (s, args.dt)
        })
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Attempted to tick after this tree has already returned Status::Success or Status::Failure."
            )
        })?;

        match status {
            Status::Success => {
                println!("Behavior tree completed successfully!");
                break;
            }
            Status::Running => {}
            Status::Failure => {
                break;
            }
        }
    }

    let count = bt.blackboard().get("counter").ok_or_else(|| {
        anyhow::anyhow!("counter is missing from the blackboard")
    })?;
    assert_eq!(*count, 1);

    bt.reset_bt();

    Ok(())
}
// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the bonsai bt example through its entry point.
    #[test]
    fn test_bonsai_bt_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn test_process_action() {
        let mut val = 0;
        let res = process_action(Actions::Inc, &mut val);
        assert_eq!(res, Success);
        assert_eq!(val, 1);

        let res = process_action(Actions::Dec, &mut val);
        assert_eq!(res, Success);
        assert_eq!(val, 0);
    }
}
