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

/// Returns the count after applying `event`.
///
/// The logic is a plain function, so it is testable without opening a window.
fn apply_event(count: i32, event: &AppEvent) -> i32 {
    match event {
        AppEvent::Increment => count.saturating_add(1),
        AppEvent::Decrement => count.saturating_sub(1),
    }
}

impl Model for AppData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| {
            self.count
                .update(|count| *count = apply_event(*count, app_event));
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Incrementing and decrementing change the count by one.
    #[test]
    fn events_change_the_count_by_one() {
        assert_eq!(apply_event(0, &AppEvent::Increment), 1);
        assert_eq!(apply_event(0, &AppEvent::Decrement), -1);
    }

    /// A sequence of events accumulates.
    #[test]
    fn events_accumulate() {
        let events = [
            AppEvent::Increment,
            AppEvent::Increment,
            AppEvent::Decrement,
        ];
        let total = events.iter().fold(0, apply_event);
        assert_eq!(total, 1);
    }

    /// The count saturates instead of overflowing.
    #[test]
    fn count_saturates_at_the_limits() {
        assert_eq!(apply_event(i32::MAX, &AppEvent::Increment), i32::MAX);
        assert_eq!(apply_event(i32::MIN, &AppEvent::Decrement), i32::MIN);
    }
}
