// ANCHOR: example
//! A small Leptos counter example showing reactive signals and view mounting.
//!
//! This example is browser-targeted, so the default build path remains a
//! lightweight placeholder unless the `leptos` feature is enabled.

#[cfg(feature = "leptos-support")]
use leptos::prelude::*;

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

#[cfg(feature = "leptos-support")]
fn main() {
    mount_to_body(|| view! { <App/> });
}

#[cfg(not(feature = "leptos-support"))]
fn main() {
    println!(
        "Leptos example is browser-targeted and available with the `leptos` feature."
    );
}

// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// // [Implement Leptos reactive counter and view mounting](https://github.com/john-cd/rust_howto/issues/867)
// // <https://leptos.dev/>
// // <https://github.com/leptos-rs/awesome-leptos>
