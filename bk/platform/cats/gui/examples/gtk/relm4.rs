// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A counter application with `relm4`.
//!
//! Relm4 builds GTK 4 apps in the Elm style: a model holds the state, messages
//! describe what happened, and an update function changes the state, after
//! which the view refreshes. This keeps state changes in one place.

use gtk::prelude::*;
use relm4::prelude::*;

/// The model of the counter.
struct CounterModel {
    /// The current count.
    value: i32,
}

/// The messages that the counter handles.
#[derive(Debug)]
enum CounterInput {
    /// Increases the count.
    Increment,

    /// Decreases the count.
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for CounterModel {
    /// Alias for `i32` used to represent init.
    type Init = i32;
    /// Alias for `CounterInput` used to represent input.
    type Input = CounterInput;
    /// Alias for `()` used to represent output.
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Relm4 Counter"),
            set_default_size: (300, 100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 10,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.value),
                },

                gtk::Button {
                    set_label: "Increment",
                    connect_clicked => CounterInput::Increment,
                },

                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked => CounterInput::Decrement,
                },

                gtk::Label {
                    #[watch]
                    set_label: if model.value % 2 == 0 { "Even" } else { "Odd" },
                }
            }
        }
    }

    /// Initializes the counter from its input and constructs the generated GTK widgets.
    fn init(
        init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { value: init };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    /// Applies a counter message; watched widget properties refresh from model state.
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        self.apply(message);
    }
}

impl CounterModel {
    /// Updates the count for a message.
    fn apply(&mut self, message: CounterInput) {
        match message {
            CounterInput::Increment => self.value += 1,
            CounterInput::Decrement => self.value -= 1,
        }
    }
}

/// Runs the application.
fn main() {
    // Keep Clap's subcommand out of GApplication's file arguments.
    let app = RelmApp::new("relm4.example.counter").with_args(vec!["relm4-counter".into()]);
    app.run::<CounterModel>(0);
}
// ANCHOR_END: example

/// Opens the Relm4 counter application.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the relm4 example through its entry point.
    #[test]
    #[ignore = "requires an interactive GTK session"]
    fn test_relm4_example_entry_point() {
        main();
    }

    /// Updates the displayed count for each received message.
    #[test]
    fn messages_update_the_count() {
        let mut model = CounterModel { value: 0 };
        model.apply(CounterInput::Increment);
        model.apply(CounterInput::Increment);
        assert_eq!(model.value, 2);
        model.apply(CounterInput::Decrement);
        model.apply(CounterInput::Decrement);
        model.apply(CounterInput::Decrement);
        assert_eq!(model.value, -1);
    }
}
