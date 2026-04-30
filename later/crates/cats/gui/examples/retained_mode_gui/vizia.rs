#![allow(dead_code)]
// ANCHOR: example
use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::Increment => {
                self.count += 1;
            }
            AppEvent::Decrement => {
                self.count -= 1;
            }
        });
    }
}

pub fn main() {
    let _ = Application::new(|cx| {
        AppData { count: 0 }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, AppData::count)
                .font_size(30.0);

            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Increment"))
                    .on_press(|cx| cx.emit(AppEvent::Increment));

                Button::new(cx, |cx| Label::new(cx, "Decrement"))
                    .on_press(|cx| cx.emit(AppEvent::Decrement));
            });
        });
    })
    .title("Vizia Counter")
    .inner_size((400, 150))
    .run();
}
// ANCHOR_END: example
