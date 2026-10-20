#![allow(dead_code)]
// ANCHOR: example
use xilem::EventLoop;
use xilem::WindowOptions;
use xilem::Xilem;
use xilem::view::Axis;
use xilem::view::button;
use xilem::view::flex;
use xilem::view::label;

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

pub fn run() -> Result<(), xilem::winit::error::EventLoopError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires an interactive example runtime"]
    fn test_main() -> Result<(), xilem::winit::error::EventLoopError> {
        main()
    }
}
