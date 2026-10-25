#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Embassy is an async runtime designed for embedded systems in Rust. It
//! provides async capabilities to work with hardware, making it easier to
//! build responsive and efficient applications for embedded devices.
//!
//! Example for a microcontroller with embedded Rust support (e.g., an
//! STM32F4 board).
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! embassy = { version = "0.7", features = ["alloc", "std"] }
//! cortex-m = "0.7"
//! cortex-m-rt = "0.7"
//! panic-halt = "0.2"
//!
//! [build-dependencies]
//! cortex-m-rtic = "0.7"

#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy::executor::Executor;
use embassy::executor::Spawner;
use embassy::time::Duration;
use embassy::time::Timer;
use panic_halt as _;

/// A task that blinks an LED forever.
#[embassy::task]
async fn blink_task() {
    // Configure your LED pin here
    // This is pseudo-code, you'll need to use the actual HAL for your
    // microcontroller.
    let led = configure_led_pin();

    loop {
        // Turn LED on.
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        // Turn LED off.
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

/// The executor that runs the tasks.
static EXECUTOR: Executor = Executor::new();

/// Starts the executor and spawns the blink task.
#[entry]
fn main() -> ! {
    // Initialize the executor
    EXECUTOR.run(|spawner| {
        spawner.spawn(blink_task()).unwrap();
    })
}

/// Configures the pin of the LED.
fn configure_led_pin() -> LedPin {
    // This is where you'd configure your LED pin
    // Replace with actual hardware setup code
    LedPin::new()
}

/// A placeholder for the pin of an LED.
struct LedPin;

impl LedPin {
    /// Creates the pin.
    fn new() -> Self {
        // Initialize your LED pin here
        LedPin
    }

    /// Sets the pin high, which turns the LED on.
    fn set_high(&self) {
        // Set the LED pin high
    }

    /// Sets the pin low, which turns the LED off.
    fn set_low(&self) {
        // Set the LED pin low
    }
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() -> ! {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires embassy crate"]
    fn test_main() {
        main();
    }
}
// [finish](https://github.com/john-cd/rust_howto/issues/751)
