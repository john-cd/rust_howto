#![allow(dead_code)]
// TODO Reviewed ??-??-2026
// ANCHOR: example
//! A counter application with `vizia`.

use vizia::prelude::*;

/// The data of the application.
#[derive(Lens)]
pub struct AppData {
    /// The current count.
    count: i32,
}

/// The events that the application handles.
pub enum AppEvent {
    /// Increases the count.
    Increment,
    /// Decreases the count.
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

/// Runs the application.
pub fn main() {
    let _ = Application::new(|cx| {
        AppData { count: 0 }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, AppData::count).font_size(30.0);

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

/// Runs the example.
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
