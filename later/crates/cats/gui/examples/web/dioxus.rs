// ANCHOR: example
use dioxus::prelude::*;

// Dioxus is a cross-platform UI framework for Rust.

pub fn main() {
    // Launch the application
    #[cfg(feature = "dioxus")]
    dioxus::launch(App);
}

#[cfg(feature = "dioxus")]
#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "Counter: {count}" }
            button { onclick: move |_| count += 1, "+" }
            button { onclick: move |_| count -= 1, "-" }
        }
    }
}
// ANCHOR_END: example
