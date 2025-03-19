// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use gtk::Application;
// use gtk::ApplicationWindow;
// use gtk::Button;
// use gtk::Label;
// use gtk::Orientation;
// use gtk::prelude::*;

// fn main() {
//     // Create a new application
//     let application = Application::builder()
//         .application_id("org.example.gtk4_counter")
//         .build();

//     application.connect_activate(|app| {
//         // Create a new window
//         let window = ApplicationWindow::builder()
//             .application(app)
//             .title("GTK4 Counter")
//             .build();

//         // Create a vertical box to hold the widgets
//         let vbox = gtk::Box::new(Orientation::Vertical, 6); // 6 is the
// spacing between widgets

//         // Create widgets using the builder pattern, which is the recommended
//         // way in GTK4. Create the counter label
//         let counter_label = Label::builder().label("Counter: 0").build();
//         vbox.append(&counter_label);

//         // Create the increment button
//         let increment_button = Button::builder().label("Increment").build();
//         vbox.append(&increment_button);

//         // Create the decrement button
//         let decrement_button = Button::builder().label("Decrement").build();
//         vbox.append(&decrement_button);

//         // Create the conditional label
//         let conditional_label = Label::builder().label("").build();
//         vbox.append(&conditional_label);

//         // Store the counter value
//         let mut counter_value = 0;

//         // Connect the increment button to a callback
//         increment_button.connect_clicked(move |_| {
//             counter_value += 1;
//             counter_label.set_label(&format!("Counter: {}", counter_value));
//             // Connect the button clicks to closures that update the counter
//             // value and the label.
//             update_conditional_label(&conditional_label, counter_value);
//         });

//         // Connect the decrement button to a callback
//         decrement_button.connect_clicked(move |_| {
//             counter_value -= 1;
//             counter_label.set_label(&format!("Counter: {}", counter_value));
//             update_conditional_label(&conditional_label, counter_value);
//         });

//         update_conditional_label(&conditional_label, counter_value);

//         // Add the box to the window
//         window.set_child(Some(&vbox));

//         // Present the window
//         window.present();
//     });

//     application.run();
// }

// // Implements the conditional rendering logic within the button click
// // handlers, updating the conditional_label text and background
// // color.
// fn update_conditional_label(label: &Label, value: i32) {
//     if value % 2 == 0 {
//         label.set_label("Counter is even!");
//     } else {
//         label.set_label("Counter is odd!");
//     }
// }

pub fn main() {}
// // [WIP finish; review https://gtk-rs.org/](https://github.com/john-cd/rust_howto/issues/780)
