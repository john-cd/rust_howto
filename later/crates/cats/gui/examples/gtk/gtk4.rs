#![allow(dead_code)]
// ANCHOR: example
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation};

pub fn main() {
    let app = Application::builder()
        .application_id("org.example.gtk4_counter")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GTK4 Counter")
        .default_width(300)
        .default_height(200)
        .build();

    let vbox = gtk::Box::new(Orientation::Vertical, 10);
    vbox.set_margin_top(10);
    vbox.set_margin_bottom(10);
    vbox.set_margin_start(10);
    vbox.set_margin_end(10);

    let counter_value = std::sync::Arc::new(std::sync::Mutex::new(0));

    let label = Label::builder()
        .label("Counter: 0")
        .build();

    let button_inc = Button::builder()
        .label("Increment")
        .build();

    let button_dec = Button::builder()
        .label("Decrement")
        .build();

    let conditional_label = Label::builder()
        .label("Counter is even!")
        .build();

    button_inc.connect_clicked({
        let counter_value = counter_value.clone();
        let label = label.clone();
        let conditional_label = conditional_label.clone();
        move |_| {
            let mut value = counter_value.lock().unwrap();
            *value += 1;
            label.set_text(&format!("Counter: {}", *value));
            update_conditional_label(&conditional_label, *value);
        }
    });

    button_dec.connect_clicked({
        let counter_value = counter_value.clone();
        let label = label.clone();
        let conditional_label = conditional_label.clone();
        move |_| {
            let mut value = counter_value.lock().unwrap();
            *value -= 1;
            label.set_text(&format!("Counter: {}", *value));
            update_conditional_label(&conditional_label, *value);
        }
    });

    vbox.append(&label);
    vbox.append(&button_inc);
    vbox.append(&button_dec);
    vbox.append(&conditional_label);

    window.set_child(Some(&vbox));
    window.present();
}

fn update_conditional_label(label: &Label, value: i32) {
    if value % 2 == 0 {
        label.set_text("Counter is even!");
    } else {
        label.set_text("Counter is odd!");
    }
}
// ANCHOR_END: example
