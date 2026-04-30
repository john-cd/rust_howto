#![allow(dead_code)]
// ANCHOR: example
use relm4::prelude::*;
use gtk::prelude::*;

struct CounterModel {
    value: i32,
}

#[derive(Debug)]
enum CounterInput {
    Increment,
    Decrement,
}

#[relm4::component(pub)]
impl SimpleComponent for CounterModel {
    type Init = i32;
    type Input = CounterInput;
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

    fn init(
        value: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = CounterModel { value };
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            CounterInput::Increment => self.value += 1,
            CounterInput::Decrement => self.value -= 1,
        }
    }
}

pub fn main() {
    let app = RelmApp::new("relm4.example.counter");
    app.run::<CounterModel>(0);
}
// ANCHOR_END: example
