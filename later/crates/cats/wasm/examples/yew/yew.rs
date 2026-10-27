#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! The main function to render the App component.

use yew::prelude::*;

/// # App Component
///
/// This is a simple counter application built with Yew.
///
/// The `function_component` attribute creates a function component from a
/// normal Rust function. Functions with this attribute must return `Html` and
/// can optionally receive a reference to the props.
#[function_component(App)]
fn app() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
        // `Callback::from` is used to create a callback that will be triggered
        // when the button is clicked.
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
        // `Callback::from` is used to create a callback that will be triggered
        // when the button is clicked.
    };

    html! {
        <div>
            <h1>{ "Yew Counter Example" }</h1>
            <p>{ format!("Counter: {}", *counter) }</p>
            <button onclick={increment}>{ "+" }</button>
            <button onclick={decrement}>{ "-" }</button>
            /* Example of conditional rendering */
            {
                if *counter > 5 {
                  html! { <p style="color: green;">{ "Counter is greater than 5!" }</p> }
                } else if *counter < -5 {
                    html! { <p style="color: red;">{ "Counter is less than -5!" }</p> }
                } else {
                    html! {}
                }
            }
        </div>
    }
}

/// Runs the yew example.
fn main() {
    #[cfg(target_arch = "wasm32")]
    yew::Renderer::<App>::new().render();

    #[cfg(not(target_arch = "wasm32"))]
    println!("Build for wasm32-unknown-unknown and serve with Trunk.");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    /// Runs the yew example through its entry point.
    #[test]
    #[ignore = "requires wasm example runtime"]
    fn yew_example_entry_point_runs() {
        main();
    }

    /// Renders the initial counter state to HTML with Yew's server renderer.
    #[tokio::test]
    async fn server_render_shows_counter() {
        let html = yew::ServerRenderer::<App>::new().render().await;
        assert!(html.contains("Yew Counter Example"));
        assert!(html.contains("Counter: 0"));
    }
}
