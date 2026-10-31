// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A counter application with `xilem`.
//!
//! Xilem builds the interface from a function of the state: it returns a
//! lightweight description of the view, and the framework compares it with the
//! previous one and updates only what changed. This is similar to React in the
//! web world.

use xilem::EventLoop;
use xilem::WindowOptions;
use xilem::Xilem;
use xilem::view::Axis;
use xilem::view::flex;
use xilem::view::label;
use xilem::view::text_button;

/// The state of the counter.
#[derive(Default)]
struct Counter {
    /// The current count.
    num: i32,
}

impl Counter {
    /// Adds one to the count.
    fn increment(&mut self) {
        self.num += 1;
    }

    /// Subtracts one from the count.
    fn decrement(&mut self) {
        self.num -= 1;
    }
}

/// Returns the view of the application for the current state.
fn app_logic(data: &mut Counter) -> impl xilem::WidgetView<Counter> + use<> {
    flex(
        Axis::Vertical,
        (
            label(format!("Count: {}", data.num)),
            text_button("Increment", Counter::increment),
            text_button("Decrement", Counter::decrement),
        ),
    )
}

/// Runs the Xilem application event loop.
fn run_app() -> Result<(), winit::error::EventLoopError> {
    let app = Xilem::new_simple(
        Counter::default(),
        app_logic,
        WindowOptions::new("Xilem Counter"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}

// ANCHOR_END: example

/// Runs the counter example as a standalone Cargo target or book test.
pub fn run() -> Result<(), winit::error::EventLoopError> {
    run_app()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the counter when an interactive desktop session is available.
    #[test]
    #[ignore = "requires an interactive example runtime"]
    fn test_xilem_example_entry_point()
    -> Result<(), winit::error::EventLoopError> {
        run_app()
    }

    /// Checks the counter logic behind the buttons.
    #[test]
    fn counter_increments_and_decrements() {
        let mut counter = Counter::default();
        counter.increment();
        counter.increment();
        assert_eq!(counter.num, 2);
        counter.decrement();
        counter.decrement();
        counter.decrement();
        assert_eq!(counter.num, -1);
    }
}
