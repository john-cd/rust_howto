// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A web application with `dioxus`.

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

/// Helper function to execute the example entry point.
pub fn run() {
    main();
}
// ANCHOR_END: example

#[cfg(all(test, feature = "dioxus"))]
mod tests {
    use super::*;

    /// Verifies that the component renders the expected HTML via Server-Side
    /// Rendering.
    #[test]
    fn test_dioxus_ssr_render() {
        let mut dom = VirtualDom::new(App);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("Count: 0"));
    }
}
