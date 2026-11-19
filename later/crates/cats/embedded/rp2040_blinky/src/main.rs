//! Blinks the original Raspberry Pi Pico's onboard LED.
//!
//! TODO Reviewed ??-??-2026.

#![no_std]
#![no_main]

// ANCHOR: example
use embassy_rp::gpio::Level;
use embassy_rp::gpio::Output;
use embassy_time::Ticker;
use panic_halt as _;

/// Starts the RP2040 peripherals and toggles the LED twice per second.
#[embassy_executor::main(
    executor = "embassy_rp::executor::Executor",
    entry = "cortex_m_rt::entry"
)]
async fn main(_spawner: embassy_executor::Spawner) {
    let peripherals = embassy_rp::init(Default::default());
    let mut led = Output::new(peripherals.PIN_25, Level::Low);
    let mut ticks = Ticker::every(embassy_time::Duration::from_millis(500));

    loop {
        ticks.next().await;
        led.toggle();
    }
}
// ANCHOR_END: example
