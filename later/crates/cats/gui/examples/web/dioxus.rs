// TODO Reviewed ??-??-2026
// ANCHOR: example
#[cfg(feature = "dioxus")]
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

pub fn run() {
    main();
}
// ANCHOR_END: example
