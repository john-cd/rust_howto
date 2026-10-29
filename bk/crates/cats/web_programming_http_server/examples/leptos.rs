// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A small Leptos counter example showing reactive signals and view mounting.
//!
//! This example is browser-targeted, so the default build path remains a
//! lightweight placeholder unless the `leptos` feature is enabled.

#[cfg(feature = "leptos-support")]
use leptos::prelude::*;

/// A counter that displays whether its count is even or odd.
#[cfg(feature = "leptos-support")]
#[component] // Annotate a function, so it can be used as a component.
fn App() -> impl IntoView {
    // Reactive signal for the counter.
    // Make the UI automatically update when the count changes.
    let (count, set_count) = signal(0);
    // Derived signal for even/odd status.
    // It recalculates automatically based on the count signal,
    // avoiding manual updates.
    let is_even = move || count.get() % 2 == 0;
    // The view macro defines the UI structure.
    view! {
        <main>
            <h1>"Leptos Counter Example"</h1>

            <p>"Count: " {move || count.get()}</p>
            <button on:click=move |_| set_count.update(|n| *n += 1)>
                "+"
            </button>
            <button on:click=move |_| set_count.update(|n| *n -= 1)>
                "-"
            </button>

            <p>{move || if is_even() { "even" } else { "odd" }}</p>
            <p>{move || if count.get() > 5 {
                "Count is greater than 5!"
            } else {
                "Keep counting..."
            }}</p>
        </main>
    }
}

/// Mounts the app in the body of the page.
#[cfg(feature = "leptos-support")]
fn main() {
    mount_to_body(|| view! { <App/> });
}

/// Prints a message when the `leptos-support` feature is not enabled.
#[cfg(not(feature = "leptos-support"))]
fn main() {
    //     // FIXME
    //     mount_to_body(App);
    println!(
        "leptos example: a reactive web framework for building web UIs (see full implementation in the book)"
    );
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    // output-only: the example only mounts a view or prints a message; see the
    // other tests for its logic.
    /// Verifies that the leptos example's main path succeeds.
    /// Runs the leptos example through its entry point.
    #[test]
    fn test_leptos_example_entry_point() {
        #[cfg(not(feature = "leptos-support"))]
        {
            super::main();
        }

        #[cfg(feature = "leptos-support")]
        {
            use leptos::prelude::Owner;

            // Construct the reactive component without mounting a browser DOM.
            // HTML serialization requires Leptos's separate SSR feature.
            Owner::new().with(|| {
                let _view = super::App();
            });
        }
    }
}
// // [Implement Leptos reactive counter and view mounting](https://github.com/john-cd/rust_howto/issues/867)
// // <https://leptos.dev/>
// // <https://github.com/leptos-rs/awesome-leptos>
