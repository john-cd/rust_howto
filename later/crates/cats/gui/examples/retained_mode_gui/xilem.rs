#![allow(dead_code)]
// ANCHOR: example
use xilem::view::{button, flex, label, Axis};
use xilem::{EventLoop, WindowOptions, Xilem};

#[derive(Default)]
struct Counter {
    num: i32,
}

fn app_logic(data: &mut Counter) -> impl xilem::WidgetView<Counter> + use<> {
    flex(
        Axis::Vertical,
        (
            label(format!("{}", data.num)),
            button(label("increment"), |data: &mut Counter| data.num += 1),
            button(label("decrement"), |data: &mut Counter| data.num -= 1),
        ),
    )
}

pub fn main() -> Result<(), xilem::winit::error::EventLoopError> {
    let app = Xilem::new_simple(
        Counter::default(),
        app_logic,
        WindowOptions::new("Xilem Counter"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
// ANCHOR_END: example
