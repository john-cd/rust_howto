// ANCHOR: example
use xilem::EventLoop;
use xilem::WidgetView;
use xilem::Xilem;
use xilem::view::button;
use xilem::view::flex;
use xilem::view::label;

#[derive(Default)]
struct Counter {
    num: i32,
}

fn app_logic(data: &mut Counter) -> impl WidgetView<Counter> + 'static {
    flex((
        label(format!("Count: {}", data.num)),
        button("Increment", |data: &mut Counter| data.num += 1),
    ))
}

fn main() -> Result<(), winit::error::EventLoopError> {
    let app = Xilem::new(Counter::default(), app_logic);
    let event_loop = EventLoop::with_user_event();
    app.run_windowed(event_loop, "Xilem Counter".into())
}
// ANCHOR_END: example

pub fn run() -> Result<(), winit::error::EventLoopError> {
    main()
}

// TODO add a test
