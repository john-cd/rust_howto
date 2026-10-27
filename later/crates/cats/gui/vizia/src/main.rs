// ANCHOR: example
//! A counter application using Vizia 0.4's signal-based API.

use vizia::prelude::*;

/// The data of the application.
pub struct AppData {
    /// The current count.
    count: Signal<i32>,
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
                self.count.update(|count| *count += 1);
            }
            AppEvent::Decrement => {
                self.count.update(|count| *count -= 1);
            }
        });
    }
}

/// Runs the Vizia counter window.
///
/// # Errors
///
/// Returns an error if Vizia cannot initialize or run the event loop.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    Application::new(|cx| {
        let count = Signal::new(0);
        AppData { count }.build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, count).font_size(30.0);

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
    .run()?;

    Ok(())
}
// ANCHOR_END: example
