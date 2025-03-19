// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// // Embassy is an async runtime designed for embedded systems in Rust. It
// // provides async capabilities to work with hardware, making it easier to
// build // responsive and efficient applications for embedded devices.

// // In `Cargo.toml`:
// // [dependencies]
// // embassy = { version = "0.7", features = ["alloc", "std"] }
// // cortex-m = "0.7"
// // cortex-m-rt = "0.7"
// // panic-halt = "0.2"

// // [build-dependencies]
// // cortex-m-rtic = "0.7"

// // Example for a microcontroller with embedded Rust support (e.g., an STM32F4
// // board).

// #![no_std]
// #![no_main]

// use core::future::Future;

// use cortex_m_rt::entry;
// use embassy::executor::Executor;
// use embassy::executor::Spawner;
// use embassy::time::Duration;
// use embassy::time::Timer;
// use panic_halt as _;

// #[embassy::task]
// async fn blink_task() {
//     // Configure your LED pin here
//     // This is pseudo-code, you'll need to use the actual HAL for your
//     // microcontroller
//     let led = configure_led_pin();

//     loop {
//         // Turn LED on
//         led.set_high();
//         Timer::after(Duration::from_millis(500)).await;

//         // Turn LED off
//         led.set_low();
//         Timer::after(Duration::from_millis(500)).await;
//     }
// }

// static EXECUTOR: Executor = Executor::new();

// #[entry]
// fn main() -> ! {
//     // Initialize the executor
//     EXECUTOR.run(|spawner| {
//         spawner.spawn(blink_task()).unwrap();
//     })
// }

// fn configure_led_pin() -> LedPin {
//     // This is where you'd configure your LED pin
//     // Replace with actual hardware setup code
//     LedPin::new()
// }

// struct LedPin;

// impl LedPin {
//     fn new() -> Self {
//         // Initialize your LED pin here
//         LedPin
//     }

//     fn set_high(&self) {
//         // Set the LED pin high
//     }

//     fn set_low(&self) {
//         // Set the LED pin low
//     }
// }

// #[test]
// fn test() {
//     main();
// }
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/751)
