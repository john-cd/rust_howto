// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! The main function to render the App component.

use yew::prelude::*;

/// A simple counter component built with Yew.
///
/// The component uses state hooks and callbacks to update its rendered HTML.
#[component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };

    html! {
        <div>
            <h1>{ "Yew Counter Example" }</h1>
            <p>{ format!("Counter: {}", *counter) }</p>
            <button onclick={increment}>{ "+" }</button>
            <button onclick={decrement}>{ "-" }</button>
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

/// Runs the Yew example in the browser target.
fn main() {
    #[cfg(target_arch = "wasm32")]
    yew::Renderer::<App>::new().render();

    #[cfg(not(target_arch = "wasm32"))]
    println!("Build for wasm32-unknown-unknown and serve with Trunk.");
}
// ANCHOR_END: example

/// Runs the Yew example for the crate's example harness.
pub fn run() {
    main();
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;

    /// Renders the initial counter state to HTML with Yew's server renderer.
    #[tokio::test]
    async fn server_render_shows_counter() {
        let html = yew::ServerRenderer::<App>::new().render().await;
        assert!(html.contains("Yew Counter Example"));
        assert!(html.contains("Counter: 0"));
    }
}
