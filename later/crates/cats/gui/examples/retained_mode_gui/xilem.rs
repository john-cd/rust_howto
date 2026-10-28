#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A counter application with `xilem`.

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

/// Returns the view of the application for the current state.
fn app_logic(data: &mut Counter) -> impl xilem::WidgetView<Counter> + use<> {
    flex(
        Axis::Vertical,
        (
            label(format!("Count: {}", data.num)),
            text_button("Increment", |data: &mut Counter| data.num += 1),
            text_button("Decrement", |data: &mut Counter| data.num -= 1),
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

/// Starts the example as a standalone Cargo target.
fn main() -> Result<(), winit::error::EventLoopError> {
    run_app()
}
// ANCHOR_END: example

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

    #[test]
    fn test_main_entry_point() {
        if let Err(err) = main() {
            panic!("main failed: {err:?}");
        }
    }
}
