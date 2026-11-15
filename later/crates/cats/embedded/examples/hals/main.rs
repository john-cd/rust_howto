// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Starts the hals application and connects its top-level components.
//! The executable uses this module to initialize services and hand control to the application.

use embedded_hal::digital::ErrorType;
use embedded_hal::digital::OutputPin;

#[derive(Default)]
struct MockPin {
    /// Flag indicating whether the mock pin satisfies the high condition.
    high: bool,
    /// Collection of transitions values consumed by mock pin operations.
    transitions: Vec<bool>,
}

impl ErrorType for MockPin {
    type Error = core::convert::Infallible;
}

impl OutputPin for MockPin {
    /// Drives the simulated pin low and records the transition.
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.high = false;
        self.transitions.push(self.high);
        Ok(())
    }

    /// Drives the simulated pin high and records the transition.
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.high = true;
        self.transitions.push(self.high);
        Ok(())
    }
}

/// Turns an LED on and off using only the portable output-pin trait.
fn blink<P: OutputPin>(pin: &mut P) -> Result<(), P::Error> {
    pin.set_high()?;
    pin.set_low()
}

/// Runs the host-side HAL example with a simulated output pin.
fn main() -> anyhow::Result<()> {
    run_example()
}
/// Performs one host-side LED pulse with the simulated output pin.

fn run_example() -> anyhow::Result<()> {
    let mut pin = MockPin::default();
    blink(&mut pin)?;
    println!("pin transitions: {:?}", pin.transitions);
    Ok(())
}
// ANCHOR_END: example

/// Runs the book example from the example crate's module runner.
/// Starts the hals application and connects its top-level components. The executable uses this
/// module to initialize services and hand control to the application.
///
/// # Errors
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    run_example()
}

/// Verifies the example entry point succeeds with its simulated pin.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the example entry point succeeds with its simulated pin.
    /// Runs the embedded example through its entry point.
    #[test]
    fn test_embedded_example_entry_point() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn blink_changes_the_output_level() {
        let mut pin = MockPin::default();
        blink(&mut pin).expect("the mock pin accepts both output levels");
        assert_eq!(pin.transitions, [true, false]);
    }
}
