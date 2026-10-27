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

pub fn run() {
    main();
}
// ANCHOR_END: example

#[cfg(all(test, feature = "dioxus"))]
mod tests {
    use super::*;

    /// Launches the Dioxus application when a browser runtime is available.
    #[test]
    #[ignore = "requires an interactive browser application runtime"]
    fn test_dioxus_application_entry_point() {
        main();
    }
}
