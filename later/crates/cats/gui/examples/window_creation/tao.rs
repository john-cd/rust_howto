// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creating a window with `tao`.

use tao::event::Event;
use tao::event::WindowEvent;
use tao::event_loop::ControlFlow;
use tao::event_loop::EventLoop;
use tao::window::WindowBuilder;

// Tao is a cross-platform window manager library.

/// Creates a window and runs its event loop.
pub fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Tao Example")
        .build(&event_loop)
        .expect("the tao example invariant should hold");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::Resized(physical_size),
                ..
            } => {
                println!("Resized to: {physical_size:?}");
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires an interactive example runtime"]
    fn test_main() {
        main();
    }
}
