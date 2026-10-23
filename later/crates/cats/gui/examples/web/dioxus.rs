// TODO Reviewed ??-??-2026

// ANCHOR: example
//! A web application with `dioxus`.

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;

// Dioxus is a cross-platform UI framework for Rust.

/// Launches the application.
pub fn main() {
    // Launch the application
    #[cfg(feature = "dioxus")]
    dioxus::launch(App);
}

/// The root component of the application.
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

/// Runs the example.
pub fn run() {
    main();
}
// ANCHOR_END: example
