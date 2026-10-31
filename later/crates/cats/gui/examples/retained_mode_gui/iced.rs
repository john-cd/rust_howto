// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A counter application with `iced`.
//!
//! Iced follows the Elm architecture: the state is a struct, user actions
//! become messages, an update function applies them, and a view function builds
//! the interface from the state. Data flows one way, which makes behaviour easy
//! to follow and test.

use iced::Alignment;
use iced::Element;
use iced::Length;
use iced::widget::button;
use iced::widget::center;
use iced::widget::column;
use iced::widget::container;
use iced::widget::text;

/// Runs the iced example.
fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

/// The state of the counter.
#[derive(Default)]
struct Counter {
    /// The current count.
    value: i64,
}

/// The messages that the counter handles.
#[derive(Debug, Clone, Copy)]
enum Message {
    /// Increases the count.
    Increment,
    /// Decreases the count.
    Decrement,
}

impl Counter {
    /// Updates the count according to `message`.
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

    /// Returns the view of the counter.
    fn view(&self) -> Element<'_, Message> {
        let content = column![
            button("+").on_press(Message::Increment),
            text(self.value).size(50),
            button("-").on_press(Message::Decrement),
        ]
        .padding(20)
        .align_x(Alignment::Center)
        .spacing(10);

        container(center(content))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
// ANCHOR_END: example

pub fn run() -> iced::Result {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the iced example through its entry point.
    #[test]
    #[ignore = "opens a window"]
    fn test_iced_example_entry_point() -> iced::Result {
        main()
    }

    /// Verifies counter updates without opening a window.
    #[test]
    fn update_changes_the_counter_value() {
        let mut counter = Counter::default();

        counter.update(Message::Increment);
        assert_eq!(counter.value, 1);

        counter.update(Message::Decrement);
        assert_eq!(counter.value, 0);
    }
}
