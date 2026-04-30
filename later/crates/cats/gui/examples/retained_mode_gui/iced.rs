#![allow(dead_code)]
// ANCHOR: example
use iced::widget::{button, column, text, center, container};
use iced::{Alignment, Element, Length};

pub fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}

#[derive(Default)]
struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }

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
