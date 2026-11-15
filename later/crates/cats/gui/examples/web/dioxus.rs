// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A web application with `dioxus`.
//!
//! Dioxus builds user interfaces from components, functions that return markup
//! and re-run when their state changes, like React. The same code can target
//! the web (compiled to WebAssembly), desktop and mobile.

#[cfg(feature = "dioxus")]
use dioxus::prelude::*;

// Dioxus is a cross-platform UI framework for Rust.

/// Launches the application.
fn main() {
    // Launch the application using the platform default renderer.
    #[cfg(feature = "dioxus")]
    dioxus::launch(App);
}

/// The root component of the application.
#[cfg(feature = "dioxus")]
#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "Count: {count}" }
        button { onclick: move |_| count += 1, "+" }
    }
}
// ANCHOR_END: example

/// Runs the example through its program entry point.
pub fn run() {
    main();
}

/// Launches the app: with the `dioxus` feature this opens a window, so it is skipped; without it
/// `main` does nothing.
#[cfg(test)]
mod tests {
    use super::*;

    /// Launches the app: with the `dioxus` feature this opens a window, so it
    /// is skipped; without it `main` does nothing.
    /// Rendering.
    #[test]
    #[cfg_attr(feature = "dioxus", ignore = "launches the Dioxus renderer")]
    fn test_main_entry_point() {
        main();
    }

    /// Rendering.
    #[test]
    #[cfg(feature = "dioxus")]
    fn test_dioxus_ssr_render() {
        let mut dom = VirtualDom::new(App);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Count: 0"));
    }
}
