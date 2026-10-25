//! Starts the target-specific Embassy task example.
// TODO Reviewed ??-??-2026.

mod embassy;
use embassy_executor::Spawner;

// ANCHOR: example
/// Starts the sensor and processing tasks on the selected Embassy executor.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    embassy::spawn_tasks(spawner);
}
// ANCHOR_END: example
