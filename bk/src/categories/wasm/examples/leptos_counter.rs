// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Mounts a reactive Leptos counter in a browser.

use leptos::prelude::*;

/// A counter that displays whether its count is even or odd.
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

/// Mounts the counter in the document body.
fn main() {
    mount_to_body(|| view! { <App/> });
}
// ANCHOR_END: example
