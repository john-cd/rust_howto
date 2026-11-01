// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Shut a worker thread down cleanly when the user presses Ctrl-C.
//!
//! A signal is a notification the operating system sends to a process. Ctrl-C
//! sends SIGINT, which kills the program by default. Handling it lets threads
//! finish their work and release resources first (a graceful shutdown).

use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

/// Does a unit of work every few milliseconds until `shutdown` is set, then
/// returns how many units were completed.
fn worker(shutdown: &AtomicBool) -> u32 {
    let mut completed = 0;
    while !shutdown.load(Ordering::SeqCst) {
        // Do one unit of work, then check the flag again.
        completed += 1;
        thread::sleep(Duration::from_millis(5));
    }
    // Flush state, close files, etc. here.
    completed
}

/// Installs the Ctrl-C handler, starts a worker, and joins it after shutdown.
pub fn run() -> anyhow::Result<()> {
    let shutdown = Arc::new(AtomicBool::new(false));

    // The handler runs on a dedicated thread. Keep it tiny: set a flag (or
    // send on a channel) and let the main flow do the real cleanup.
    let flag = Arc::clone(&shutdown);
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst))?;

    let worker_flag = Arc::clone(&shutdown);
    let handle = thread::spawn(move || worker(&worker_flag));

    // A real program would block here until the handler fires, for example by
    // waiting on a channel. This example requests the shutdown itself so that
    // it terminates without a keyboard.
    thread::sleep(Duration::from_millis(50));
    shutdown.store(true, Ordering::SeqCst);

    let completed = handle
        .join()
        .map_err(|_| anyhow::anyhow!("the worker thread panicked"))?;
    println!("worker completed {completed} units before shutting down");
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn worker_stops_when_the_flag_is_set() {
        let shutdown = Arc::new(AtomicBool::new(false));
        let worker_flag = Arc::clone(&shutdown);
        let handle = thread::spawn(move || worker(&worker_flag));
        thread::sleep(Duration::from_millis(20));
        shutdown.store(true, Ordering::SeqCst);
        // Joining succeeds only if the worker noticed the flag and returned.
        assert!(handle.join().is_ok());
    }

    #[test]
    fn worker_does_nothing_if_already_shut_down() {
        let shutdown = AtomicBool::new(true);
        assert_eq!(worker(&shutdown), 0);
    }

    #[test]
    fn run_completes() -> anyhow::Result<()> {
        run()
    }
}
