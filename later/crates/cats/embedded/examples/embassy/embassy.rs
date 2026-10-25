#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A target-specific Embassy task sketch.
//!
//! Add this task to a firmware crate that selects an Embassy executor platform,
//! chip-specific HAL, and matching Embassy time driver.

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;

/// Bounded sensor-message channel for tasks on a single-core executor.
static SENSOR_VALUES: Channel<NoopRawMutex, u32, 4> = Channel::new();

/// Sends a placeholder sample periodically to the processing task.
#[embassy_executor::task]
async fn send_sensor_sample() {
    loop {
        let sample = 42; // Replace with a reading from the selected HAL.
        SENSOR_VALUES.send(sample).await;
        Timer::after_secs(1).await;
    }
}

/// Receives sensor samples and applies application-specific processing.
#[embassy_executor::task]
async fn process_sensor_values() {
    loop {
        let _reading = SENSOR_VALUES.receive().await;
        // Update application state or a peripheral with the reading.
    }
}

/// Starts the periodic sensor and processing tasks on the supplied executor.
pub fn spawn_tasks(spawner: Spawner) {
    spawner.spawn(send_sensor_sample());
    spawner.spawn(process_sensor_values());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    /// Documents why the target-specific firmware sketch is not run on the
    /// host.
    #[test]
    #[ignore = "requires an MCU target, a matching HAL, and a firmware runner"]
    fn embassy_firmware_runs_on_selected_board() {}
}

// [finish](https://github.com/john-cd/rust_howto/issues/751)
