//! A counter application with `iced`.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
use iced::Alignment;
use iced::Element;
use iced::Length;
use iced::widget::button;
use iced::widget::center;
use iced::widget::column;
use iced::widget::container;
use iced::widget::text;

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

/// The state of the counter.
#[derive(Default)]
struct Counter {
    /// The current count.
    value: i32,
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

/// Runs the example.
pub fn run() -> iced::Result {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires an interactive example runtime"]
    fn test_main() -> iced::Result {
        main()
    }
}
