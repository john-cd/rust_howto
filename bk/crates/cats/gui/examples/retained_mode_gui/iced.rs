// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use iced::Element;
// use iced::Settings;
// use iced::mouse::Button;
// use iced::widget::Column;
// use iced::widget::Container;
// use iced::widget::button;

// // We define a struct Counter to hold the state of our app.
// // It includes the current count and two button states.
// #[derive(Default)]
// struct Counter {
//     count: i32,
//     increment_button: button::State,
//     decrement_button: button::State,
// }

// // The Message enum defines the types of messages our app can handle:
// // incrementing or decrementing the count.
// #[derive(Debug, Clone, Copy)]
// enum Message {
//     Increment,
//     Decrement,
// }

// impl Sandbox for Counter {
//     type Message = Message;

//     fn new() -> Self {
//         Self::default()
//     }

//     // Returns the window title.
//     fn title(&self) -> String {
//         String::from("Counter - Iced")
//     }

//     // Updates the state based on the received message (increment or
// decrement     // the count).
//     fn update(&mut self, message: Message) {
//         match message {
//             Message::Increment => {
//                 self.count += 1;
//             }
//             Message::Decrement => {
//                 self.count -= 1;
//             }
//         }
//     }

//     // Defines the layout of our app. We create a column (Column) with
// centered     // items. The current count is displayed as text. We create two
// buttons for     // incrementing and decrementing the count, and we add them
// to the column.     fn view(&mut self) -> Element<Message> {
//         let mut col = Column::new()
//             .align_items(Align::Center)
//             .push(Text::new(self.count.to_string()).size(50));

//         let increment_button =
//             Button::new(&mut self.increment_button, Text::new("+"))
//                 .on_press(Message::Increment);
//         let decrement_button =
//             Button::new(&mut self.decrement_button, Text::new("-"))
//                 .on_press(Message::Decrement);

//         col = col.push(increment_button).push(decrement_button);

//         Container::new(col)
//             .width(iced::Length::Fill)
//             .height(iced::Length::Fill)
//             .center_x()
//             .center_y()
//             .into()
//     }
// }

// fn main() -> iced::Result {
//     Counter::run(Settings::default())
// }

pub fn main() {}
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/781)
