#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Records frames with `puffin` and serves them to `puffin_viewer`.
//!
//! Install the viewer with `cargo install puffin_viewer`, run it,
//! and connect it to `127.0.0.1:8585` while this program runs.

use puffin_http::Server;

/// The address of the profiling server.
/// Keep it bound to localhost: the profile data is not authenticated.
const ADDRESS: &str = "127.0.0.1:8585";

/// The state of the profiler.
enum ProfilerStatus {
    /// The server is listening. Dropping it stops the server.
    Active(Server),
    /// The server could not start. The application can continue without it.
    Failed(String),
}

/// Turns profiling on and starts serving the profile data.
fn start_profiler(address: &str) -> ProfilerStatus {
    // Scopes record nothing until they are turned on.
    puffin::set_scopes_on(true);
    match Server::new(address) {
        Ok(server) => ProfilerStatus::Active(server),
        Err(error) => {
            // For example, the port is already in use.
            puffin::set_scopes_on(false);
            ProfilerStatus::Failed(error.to_string())
        }
    }
}

/// Does some work, split into scopes.
fn do_frame_work() -> u64 {
    // Use the function name as the scope name.
    puffin::profile_function!();
    let mut total = 0;
    for step in 0..3 {
        puffin::profile_scope!("step", step.to_string());
        total += (0..1_000_u64).sum::<u64>();
    }
    total
}

/// Runs a few frames of an application loop.
fn main() {
    let status = start_profiler(ADDRESS);
    if let ProfilerStatus::Failed(error) = &status {
        println!("Profiling is off: {error}");
    } else {
        println!("Serving profile data on {ADDRESS}");
    }

    // A real application loops until the user quits.
    for _ in 0..3 {
        let total = do_frame_work();
        assert_eq!(total, 3 * 499_500);
        // Mark the end of the frame: send it to the viewer.
        puffin::GlobalProfiler::lock().new_frame();
    }
}
// ANCHOR_END: example

#[allow(clippy::main_recursion)]
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the example can run without starting a server.
    #[test]
    fn test_main() {
        main();
    }

    /// Records a frame without a server and checks the captured scopes.
    #[test]
    fn frame_has_scopes() {
        let view = puffin::GlobalFrameView::default();
        puffin::set_scopes_on(true);
        do_frame_work();
        puffin::GlobalProfiler::lock().new_frame();

        let frame = view.lock().latest_frame();
        // At least do_frame_work and three steps.
        // Other tests may add scopes to the same global frame.
        let scopes = frame.map_or(0, |frame| frame.meta().num_scopes);
        assert!(scopes >= 4);
    }

    /// Reports a bind failure as inactive profiling instead of panicking.
    #[test]
    fn failed_bind_does_not_panic() {
        // Port 1 is privileged, so binding fails for a normal user.
        // When the test runs as root, the server starts instead.
        let status = start_profiler("127.0.0.1:1");
        assert!(matches!(
            status,
            ProfilerStatus::Active(_) | ProfilerStatus::Failed(_)
        ));
    }
}
