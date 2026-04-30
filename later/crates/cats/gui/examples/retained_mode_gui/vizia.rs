// ANCHOR: example
use vizia::prelude::*;

#[derive(Lens)]
struct AppData {
    count: i32,
}

enum AppEvent {
    Increment,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => self.count += 1,
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { count: 0 }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(cx, |cx| Label::new(cx, "Increment"))
                .on_press(|cx| cx.emit(AppEvent::Increment));
            Label::new(cx, AppData::count).width(Pixels(50.0));
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(10.0));
    })
    .title("Vizia Counter")
    .inner_size((400, 100))
    .run();
}
// ANCHOR_END: example

pub fn run() {
    main();
}
