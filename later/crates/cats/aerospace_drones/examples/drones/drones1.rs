// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Sending a flight plan to a Tello drone.
//!
//! The plan is checked before any command is sent. The `tello` crate is bound
//! to the drone through the [`Flyer`] trait, so the plan can also run against
//! a test double.
//!
//! The Tello is a small consumer quadcopter. It creates its own Wi-Fi network
//! and accepts short text commands as UDP datagrams (UDP is a connectionless
//! network protocol: each message is sent without a handshake or delivery
//! guarantee). Checking the plan first matters because a drone cannot be
//! "undone": a plan with no landing would leave it hovering until the battery
//! runs out.

use std::time::Duration;

use anyhow::Result;
use anyhow::bail;
use tello::Drone;

/// The longest total hover time the plan may contain.
// Safety limit: a small drone has only a few minutes of battery.
const MAX_HOVER: Duration = Duration::from_secs(30);

/// One step of a flight plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Step {
    /// Leave the ground.
    TakeOff,
    /// Stay in place for a while.
    Hover(Duration),
    /// Return to the ground.
    Land,
}

/// Something that accepts flight commands.
trait Flyer {
    /// Takes off.
    fn take_off(&mut self) -> Result<()>;
    /// Lands.
    fn land(&mut self) -> Result<()>;
}

impl Flyer for Drone {
    fn take_off(&mut self) -> Result<()> {
        Self::take_off(self)
            .map_err(|()| anyhow::anyhow!("the drone rejected `take_off`"))
    }

    fn land(&mut self) -> Result<()> {
        Self::land(self)
            .map_err(|()| anyhow::anyhow!("the drone rejected `land`"))
    }
}

/// Checks that a plan starts with a take-off, ends with a landing, flies only
/// once, and hovers for a safe total time.
fn validate(plan: &[Step]) -> Result<()> {
    if plan.first() != Some(&Step::TakeOff) {
        bail!("a flight plan must start with a take-off");
    }
    if plan.last() != Some(&Step::Land) {
        bail!("a flight plan must end with a landing");
    }
    let takeoffs = plan.iter().filter(|step| **step == Step::TakeOff).count();
    let landings = plan.iter().filter(|step| **step == Step::Land).count();
    if takeoffs != 1 || landings != 1 {
        bail!("a flight plan must contain one take-off and one landing");
    }
    let hover: Duration = plan
        .iter()
        .filter_map(|step| match step {
            Step::Hover(duration) => Some(*duration),
            _ => None,
        })
        .sum();
    if hover > MAX_HOVER {
        bail!("hovering for {hover:?} exceeds the limit of {MAX_HOVER:?}");
    }
    Ok(())
}

/// Validates the plan, then sends its steps in order.
fn fly(flyer: &mut impl Flyer, plan: &[Step]) -> Result<()> {
    validate(plan)?;
    for step in plan {
        match step {
            Step::TakeOff => flyer.take_off()?,
            Step::Hover(duration) => std::thread::sleep(*duration),
            Step::Land => flyer.land()?,
        }
    }
    Ok(())
}

/// Connects to a drone and flies a short plan.
fn main() -> Result<()> {
    // 192.168.10.1 is the drone's fixed address on its own Wi-Fi network and
    // 8889 is the UDP port that receives commands. The computer must be joined
    // to that Wi-Fi network first.
    let mut drone = Drone::new("192.168.10.1:8889");
    // The drone sends its video stream back to this local UDP port.
    drone.connect(11111);
    println!("Drone connected.");

    let plan = [
        Step::TakeOff,
        Step::Hover(Duration::from_secs(3)),
        Step::Land,
    ];
    fly(&mut drone, &plan)
}
// ANCHOR_END: example

/// Runs the example; it needs a real drone.
///
/// # Errors
///
/// Returns an error if the plan is invalid or the drone rejects a command.
pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Records the commands it receives instead of flying.
    #[derive(Default)]
    struct Recorder(Vec<&'static str>);

    impl Flyer for Recorder {
        fn take_off(&mut self) -> Result<()> {
            self.0.push("take_off");
            Ok(())
        }

        fn land(&mut self) -> Result<()> {
            self.0.push("land");
            Ok(())
        }
    }

    /// A valid plan sends its commands in order.
    #[test]
    fn plan_is_sent_in_order() -> Result<()> {
        let mut recorder = Recorder::default();
        fly(
            &mut recorder,
            &[Step::TakeOff, Step::Hover(Duration::ZERO), Step::Land],
        )?;
        assert_eq!(recorder.0, ["take_off", "land"]);
        Ok(())
    }

    /// Invalid plans are rejected before any command is sent.
    #[test]
    fn invalid_plans_send_nothing() {
        let too_long = Step::Hover(MAX_HOVER + Duration::from_secs(1));
        let bad_plans: [&[Step]; 5] = [
            &[],
            &[Step::Land],
            &[Step::TakeOff],
            &[Step::TakeOff, Step::TakeOff, Step::Land],
            &[Step::TakeOff, too_long, Step::Land],
        ];
        for plan in bad_plans {
            let mut recorder = Recorder::default();
            assert!(fly(&mut recorder, plan).is_err(), "{plan:?}");
            assert!(recorder.0.is_empty(), "{plan:?}");
        }
    }

    /// Flies a real drone.
    #[test]
    #[ignore = "requires real drone"]
    fn test_drones1_example_entry_point() -> Result<()> {
        run()
    }
}
