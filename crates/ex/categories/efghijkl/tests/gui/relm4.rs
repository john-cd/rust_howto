#![allow(dead_code)]

// ANCHOR: example
use gtk::glib::clone;
use gtk::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::RelmApp;
use relm4::RelmWidgetExt;
use relm4::SimpleComponent;
use relm4::gtk;

// The model type that stores the application state.
struct CounterModel {
    data: CounterData,
}
struct CounterData {
    value: i32,
    //...
}

// The message types that describe which information can be sent to update the
// model / sent from the component
#[derive(Debug)]
enum CounterInput {
    Increment,
    Decrement,
}

#[derive(Debug)]
enum CounterOutput {
    CurrentValue(i32),
}

// Cloning a widget doesn't create a new instance, but just increases the
// reference count. Widgets are kept alive automatically. Dropping widgets that
// are still used somewhere does not destroy them, but just decreases the
// reference count. Widgets are not thread-safe. Widgets don't implement Send
// and can only be used on the main thread.
struct CounterWidgets {
    // window: gtk::Window,
    // vbox: gtk::Box,
    // increment_button: gtk::Button,
    // decrement_button: gtk::Button,
    counter_label: gtk::Label,
    conditional_label: gtk::Label,
}

impl SimpleComponent for CounterModel {
    /// The type of data with which this component will be initialized.
    type Init = i32;
    /// The type of the messages that this component can receive.
    type Input = CounterInput;
    /// The type of the messages that this component can send.
    type Output = CounterOutput;
    /// The root GTK widget that this component will create.
    type Root = gtk::Window;
    /// A data structure that contains the widgets that you will need to update.
    type Widgets = CounterWidgets;

    // The Root type is the outermost widget of the app. Components can choose
    // this type freely, but the main component must use a Window.
    fn init_root() -> Self::Root {
        gtk::Window::builder()
            .title("Simple app")
            .default_width(300)
            .default_height(100)
            .build()
    }

    fn init(
        init_value: Self::Init,
        window: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let counter_model = CounterModel {
            data: CounterData { value: init_value },
        };

        let vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(5)
            .build();

        let increment_button: gtk::Button =
            gtk::Button::with_label("Increment");
        let decrement_button: gtk::Button =
            gtk::Button::with_label("Decrement");

        increment_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(CounterInput::Increment);
            }
        ));

        decrement_button.connect_clicked(clone!(
            #[strong]
            sender,
            move |_| {
                sender.input(CounterInput::Decrement);
            }
        ));

        let counter_label: gtk::Label = gtk::Label::new(Some(&format!(
            "Counter: {}",
            counter_model.data.value
        )));
        counter_label.set_margin_all(5);
        let conditional_label: gtk::Label = gtk::Label::new(None);
        conditional_label.set_margin_all(5);

        window.set_child(Some(&vbox));
        vbox.set_margin_all(5);
        vbox.append(&increment_button);
        vbox.append(&decrement_button);
        vbox.append(&counter_label);
        vbox.append(&conditional_label);

        let component_parts = ComponentParts {
            model: counter_model,
            widgets: CounterWidgets {
                // window,
                // vbox,
                // increment_button,
                // decrement_button,
                counter_label,
                conditional_label,
            },
        };

        component_parts
    }

    // Process messages and update its model.
    fn update(&mut self, input: Self::Input, sender: ComponentSender<Self>) {
        match input {
            CounterInput::Increment => {
                self.data.value += 1;
                sender
                    .output(CounterOutput::CurrentValue(self.data.value))
                    .unwrap();
            }
            CounterInput::Decrement => {
                self.data.value -= 1;
                sender
                    .output(CounterOutput::CurrentValue(self.data.value))
                    .unwrap();
            }
        }
    }

    // Update the view to represent the updated model.
    fn update_view(
        &self,
        widgets: &mut CounterWidgets,
        _sender: ComponentSender<Self>,
    ) {
        widgets
            .counter_label
            .set_label(&format!("Counter: {}", self.data.value));

        if self.data.value % 2 == 0 {
            widgets.conditional_label.set_label("Counter is even!");
        } else {
            widgets.conditional_label.set_label("Counter is odd!");
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.simple_manual");
    app.run::<CounterModel>(1);
}
// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// How to test?
// [P1](https://github.com/john-cd/rust_howto/issues/784)
// TODO https://relm4.org/book/stable/
