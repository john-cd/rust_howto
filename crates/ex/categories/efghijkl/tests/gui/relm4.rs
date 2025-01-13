// // ANCHOR: example
// use gtk::prelude::*;
// use relm4::{
//     gtk, Component, ComponentParts, ComponentSender, SimpleComponent,
// WidgetPlus, };

// struct CounterModel {
//     value: i32,
// }

// #[derive(Debug)]
// enum CounterInput {
//     Increment,
//     Decrement,
// }

// #[derive(Debug)]
// enum CounterOutput {}

// struct Counter {
//     model: CounterModel,
// }

// impl SimpleComponent for Counter {
//     type Init = ();
//     type Input = CounterInput;
//     type Output = CounterOutput;
//     type Root = gtk::Box;

//     fn init(
//         _params: Self::Init,
//         root: Self::Root,
//         sender: ComponentSender<Self>,
//     ) -> ComponentParts<Self> {
//         let model = CounterModel { value: 0 };

//         let increment_button: gtk::Button =
// root.child("increment_button").unwrap().downcast().unwrap();         let
// decrement_button: gtk::Button =
// root.child("decrement_button").unwrap().downcast().unwrap();         let
// counter_label: gtk::Label =
// root.child("counter_label").unwrap().downcast().unwrap();         let
// conditional_label: gtk::Label =
// root.child("conditional_label").unwrap().downcast().unwrap();

//         increment_button.connect_clicked(move |_| {
//             sender.input(CounterInput::Increment);
//         });

//         decrement_button.connect_clicked(move |_| {
//             sender.input(CounterInput::Decrement);
//         });

//         let component_parts = ComponentParts {
//             model,
//             widgets: CounterWidgets {
//                 root,
//                 increment_button,
//                 decrement_button,
//                 counter_label,
//                 conditional_label,
//             },
//         };

//         component_parts
//     }

//     fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>)
// {         match input {
//             CounterInput::Increment => self.model.value += 1,
//             CounterInput::Decrement => self.model.value -= 1,
//         }
//     }

//     fn view(&self, widgets: &mut CounterWidgets) {
//         widgets.counter_label.set_label(&format!("Counter: {}",
// self.model.value));

//         if self.model.value % 2 == 0 {
//             widgets.conditional_label.set_label("Counter is even!");
//
// widgets.conditional_label.override_background_color(gtk::StateFlags::NORMAL,
// Some(&gtk::gdk::RGBA{ red: 0.56, green: 0.93, blue: 0.56, alpha: 1.0 })); //
// Light Green         } else {
//             widgets.conditional_label.set_label("Counter is odd!");
//
// widgets.conditional_label.override_background_color(gtk::StateFlags::NORMAL,
// Some(&gtk::gdk::RGBA{ red: 1.0, green: 1.0, blue: 0.88, alpha: 1.0 })); //
// Light Yellow         }
//     }
// }

// struct CounterWidgets {
//     root: gtk::Box,
//     increment_button: gtk::Button,
//     decrement_button: gtk::Button,
//     counter_label: gtk::Label,
//     conditional_label: gtk::Label,
// }

// fn main() {
//     let application = gtk::Application::builder()
//         .application_id("org.example.relm4_counter")
//         .build();

//     application.connect_activate(|app| {
//         let window = gtk::ApplicationWindow::builder()
//             .application(app)
//             .title("Relm4 Counter")
//             .build();

//         let main_box = gtk::Box::builder()
//             .orientation(gtk::Orientation::Vertical)
//             .margin_top(12)
//             .margin_bottom(12)
//             .margin_start(12)
//             .margin_end(12)
//             .spacing(12)
//             .build();

//         let counter_label = gtk::Label::builder().label("Counter:
// 0").build();         counter_label.set_widget_name("counter_label");
//         let conditional_label = gtk::Label::builder().label("").build();
//         conditional_label.set_widget_name("conditional_label");
//         let increment_button =
// gtk::Button::builder().label("Increment").build();         increment_button.
// set_widget_name("increment_button");         let decrement_button =
// gtk::Button::builder().label("Decrement").build();         decrement_button.
// set_widget_name("decrement_button");

//         main_box.append(&counter_label);
//         main_box.append(&increment_button);
//         main_box.append(&decrement_button);
//         main_box.append(&conditional_label);

//         window.set_child(Some(&main_box));

//         let component = relm4::Component::<Counter>::builder()
//             .launch((), main_box)
//             .forward(app, |output| match output {});

//         window.show();
//     });

//     application.run();
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/784)
