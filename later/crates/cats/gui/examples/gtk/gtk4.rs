// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A GTK 4 application with a button and a label.
//!
//! GTK is a widely used toolkit for desktop interfaces on Linux (the base of
//! GNOME), also available on Windows and macOS. A GUI program creates widgets
//! such as buttons and labels, then connects signal handlers that run when the
//! user acts. It needs the GTK 4 libraries installed.

use gtk::Application;
use gtk::ApplicationWindow;
use gtk::Button;
use gtk::Label;
use gtk::Orientation;
use gtk::prelude::*;

/// Creates and runs the application.
fn main() {
    let app = Application::builder()
        .application_id("org.example.gtk4_counter")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

/// Builds the window of the application.
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

    let label = Label::builder().label("Counter: 0").build();

    let button_inc = Button::builder().label("Increment").build();

    let button_dec = Button::builder().label("Decrement").build();

    let conditional_label = Label::builder().label("Counter is even!").build();

    button_inc.connect_clicked({
        let counter_value = counter_value.clone();
        let label = label.clone();
        let conditional_label = conditional_label.clone();
        move |_| {
            let mut value =
                counter_value.lock().expect("mutex should not be poisoned");
            *value += 1;
            label.set_text(&format!("Counter: {}", *value));
            update_conditional_label(&conditional_label, *value);
        }
    });

    button_dec.connect_clicked({
        let label = label.clone();
        let conditional_label = conditional_label.clone();
        move |_| {
            let mut value =
                counter_value.lock().expect("mutex should not be poisoned");
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

/// Updates `label` according to `value`.
fn update_conditional_label(label: &Label, value: i32) {
    if value % 2 == 0 {
        label.set_text("Counter is even!");
    } else {
        label.set_text("Counter is odd!");
    }
}
// ANCHOR_END: example

/// Opens the GTK 4 counter application.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Activates the application on a display and clicks both buttons.
    ///
    /// Run with a display, for example `xvfb-run -a cargo test --features gtk
    /// --example gtk -- --ignored --test-threads=1`: GTK must be used from the
    /// main thread.
    #[test]
    #[ignore = "requires a GTK display, e.g. xvfb-run"]
    fn buttons_update_the_label() {
        let app = Application::builder()
            .application_id("org.example.gtk4_counter_test")
            .build();
        app.connect_activate(build_ui);
        app.register(None::<&gtk::gio::Cancellable>)
            .expect("application registers");
        app.activate();

        let window = app.windows().pop().expect("window was created");
        let vbox = window
            .child()
            .and_downcast::<gtk::Box>()
            .expect("window child is a box");
        let label = vbox
            .first_child()
            .and_downcast::<Label>()
            .expect("first child is the counter label");
        let increment = label
            .next_sibling()
            .and_downcast::<Button>()
            .expect("second child is the increment button");
        let decrement = increment
            .next_sibling()
            .and_downcast::<Button>()
            .expect("third child is the decrement button");
        let parity = decrement
            .next_sibling()
            .and_downcast::<Label>()
            .expect("fourth child is the parity label");

        increment.emit_clicked();
        assert_eq!(label.text(), "Counter: 1");
        assert_eq!(parity.text(), "Counter is odd!");
        decrement.emit_clicked();
        decrement.emit_clicked();
        assert_eq!(label.text(), "Counter: -1");
        assert_eq!(parity.text(), "Counter is odd!");
    }
}
