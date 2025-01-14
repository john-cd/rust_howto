// // ANCHOR: example
// use gtk::Application;
// use gtk::ApplicationWindow;
// use gtk::Button;
// use gtk::Label;
// use gtk::Orientation;
// use gtk::prelude::*;

// GTK4 Structure: Uses the standard GTK4 application structure with Application
// and ApplicationWindow. Widget Creation: Creates widgets using the builder
// pattern, which is the recommended way in GTK4. Layout: Uses a gtk::Box with
// vertical orientation to arrange the widgets. Spacing is added for better
// visual separation. Event Handling: Connects the button clicks to closures
// that update the counter value and the label. Conditional Rendering:
// Implements the conditional rendering logic within the button click handlers,
// updating the conditional_label text and background color. Color Handling:
// Uses override_background_color with gtk::gdk::RGBA to set background colors.
// update_conditional_label function: This function is created to avoid code
// duplication. It sets the label text and background color based on the value.
// Clearer Comments and Formatting: Added more comments and improved formatting
// for better readability.

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

//         // Create the counter label
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
//             update_conditional_label(&conditional_label, counter_value);
//         });

//         // Connect the decrement button to a callback
//         decrement_button.connect_clicked(move |_| {
//             counter_value -= 1;
//             counter_label.set_label(&format!("Counter: {}", counter_value));
//             update_conditional_label(&conditional_label, counter_value);
//         });

//         fn update_conditional_label(label: &Label, value: i32) {
//             if value % 2 == 0 {
//                 label.set_label("Counter is even!");
//                 label.override_background_color(
//                     gtk::StateFlags::NORMAL,
//                     Some(&gtk::gdk::RGBA {
//                         red: 0.56,
//                         green: 0.93,
//                         blue: 0.56,
//                         alpha: 1.0,
//                     }),
//                 ); // Light Green
//             } else {
//                 label.set_label("Counter is odd!");
//                 label.override_background_color(
//                     gtk::StateFlags::NORMAL,
//                     Some(&gtk::gdk::RGBA {
//                         red: 1.0,
//                         green: 1.0,
//                         blue: 0.88,
//                         alpha: 1.0,
//                     }),
//                 ); // Light Yellow
//             }
//         }
//         update_conditional_label(&conditional_label, counter_value);

//         // Add the box to the window
//         window.set_child(Some(&vbox));

//         // Present the window
//         window.present();
//     });

//     application.run();
// }
// // ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [P1](https://github.com/john-cd/rust_howto/issues/780)
