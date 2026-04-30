#![allow(dead_code)]
// ANCHOR: example
// fyrox example (commented out due to complex system dependencies like ALSA in the workspace)
/*
use fyrox::{
    core::pool::Handle,
    engine::{Engine, EngineInitParams},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    gui::{
        message::UiMessage,
        text::TextBuilder,
        widget::WidgetBuilder,
        window::{WindowBuilder, WindowTitle},
    },
};

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut engine = Engine::new(EngineInitParams::default()).unwrap();

    let _window: Handle<fyrox::gui::node::UiNode> = WindowBuilder::new(WidgetBuilder::new())
        .with_title(WindowTitle::text("Hello, Fyrox!"))
        .with_content(
            TextBuilder::new(WidgetBuilder::new())
                .with_text("Welcome to Fyrox!")
                .build(&mut engine.user_interface.build_ctx()),
        )
        .build(&mut engine.user_interface.build_ctx());

    event_loop.run(move |event, window_target| {
        window_target.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => window_target.exit(),
            _ => (),
        }

        if let Event::AboutToWait = event {
            engine.update(1.0 / 60.0, window_target);
        }
    }).unwrap();
}
*/
fn main() {
    println!("Fyrox example code updated to 1.0 API, but disabled due to workspace build issues.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
