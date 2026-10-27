//! Runs a bounded pair of Embassy tasks on the host executor.
// TODO Reviewed ??-??-2026

// ANCHOR: example
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::Timer;

/// Capacity for all messages sent by the example task.
const MESSAGE_COUNT: usize = 3;

/// Carries sample values from the producer task to the consumer task.
static SENSOR_VALUES: Channel<CriticalSectionRawMutex, u32, MESSAGE_COUNT> =
    Channel::new();

/// Stores each value received by the consumer for the host test to inspect.
static RECEIVED_VALUES: [AtomicU32; MESSAGE_COUNT] =
    [AtomicU32::new(0), AtomicU32::new(0), AtomicU32::new(0)];

/// Signals that the consumer received the complete message sequence.
static COMPLETED: AtomicBool = AtomicBool::new(false);

/// Signals that the timer-backed task deadline elapsed.
static TIMED_OUT: AtomicBool = AtomicBool::new(false);

/// Signals that the host executor could not start one of the tasks.
static SPAWN_FAILED: AtomicBool = AtomicBool::new(false);

/// Serializes runs because this example intentionally uses static Embassy
/// state.
static RUN_LOCK: Mutex<()> = Mutex::new(());

/// Sends three sample readings through the bounded channel.
#[embassy_executor::task]
async fn send_sensor_values() {
    for value in [10, 20, 30] {
        SENSOR_VALUES.send(value).await;
    }
}

/// Receives every sample and records its position in the sequence.
#[embassy_executor::task]
async fn receive_sensor_values() {
    for received in &RECEIVED_VALUES {
        received.store(SENSOR_VALUES.receive().await, Ordering::Release);
    }

    COMPLETED.store(true, Ordering::Release);
}

/// Ends the host run if the producer and consumer do not finish promptly.
#[embassy_executor::task]
async fn stop_after_deadline() {
    Timer::after_secs(2).await;
    TIMED_OUT.store(true, Ordering::Release);
}

/// Starts all three tasks and reports task-allocation failures.
fn spawn_tasks(spawner: Spawner) -> anyhow::Result<()> {
    let producer = send_sensor_values().map_err(|error| {
        anyhow::anyhow!("failed to spawn the sensor producer: {error:?}")
    })?;
    spawner.spawn(producer);

    let consumer = receive_sensor_values().map_err(|error| {
        anyhow::anyhow!("failed to spawn the sensor consumer: {error:?}")
    })?;
    spawner.spawn(consumer);

    let timeout = stop_after_deadline().map_err(|error| {
        anyhow::anyhow!("failed to spawn the timeout task: {error:?}")
    })?;
    spawner.spawn(timeout);
    Ok(())
}

/// Runs the producer and consumer and returns the received readings in order.
///
/// # Errors
///
/// Returns an error if the executor lock is poisoned, a task cannot be spawned,
/// or the task deadline expires before all readings arrive.
fn run_example() -> anyhow::Result<Vec<u32>> {
    let _run_guard = RUN_LOCK.lock().map_err(|_| {
        anyhow::anyhow!("the host Embassy run lock was poisoned")
    })?;
    for received in &RECEIVED_VALUES {
        received.store(0, Ordering::Relaxed);
    }
    COMPLETED.store(false, Ordering::Relaxed);
    TIMED_OUT.store(false, Ordering::Relaxed);
    SPAWN_FAILED.store(false, Ordering::Relaxed);

    let executor = Box::leak(Box::new(embassy_executor::Executor::new()));
    executor.run_until(
        |spawner| {
            if let Err(error) = spawn_tasks(spawner) {
                eprintln!("{error:#}");
                SPAWN_FAILED.store(true, Ordering::Release);
            }
        },
        || {
            COMPLETED.load(Ordering::Acquire)
                || TIMED_OUT.load(Ordering::Acquire)
                || SPAWN_FAILED.load(Ordering::Acquire)
        },
    );

    if SPAWN_FAILED.load(Ordering::Acquire) {
        anyhow::bail!("the host Embassy executor could not start all tasks");
    }
    if TIMED_OUT.load(Ordering::Acquire) {
        anyhow::bail!("the sensor tasks did not finish before the deadline");
    }

    Ok(RECEIVED_VALUES
        .iter()
        .map(|received| received.load(Ordering::Acquire))
        .collect())
}

/// Runs the host Embassy example and prints the values received by its tasks.
///
/// # Errors
///
/// Returns an error if the executor cannot start or finish the task sequence.
pub fn run() -> anyhow::Result<()> {
    let received_values = run_example()?;
    println!("Received sensor values: {received_values:?}");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::run_example;

    /// Verifies that the host consumer receives every message in order.
    #[test]
    fn host_tasks_exchange_messages_in_order() {
        let received_values = run_example()
            .expect("the host Embassy tasks should finish before the deadline");

        assert_eq!(received_values, [10, 20, 30]);
    }
}
