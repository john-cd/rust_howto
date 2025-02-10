#![allow(dead_code)]
// ANCHOR: example
use yew::prelude::*;

// The `function_component` attribute creates a function component from a normal
// Rust function. Functions with this attribute must return Html and can
// optionally receive a reference to the props.
#[function_component(App)]
fn app() -> Html {
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

fn main() {
    yew::Renderer::<App>::new().render();
}
// ANCHOR_END: example

// #[test]
// fn test() {
//     main();
// }
// [P1](https://github.com/john-cd/rust_howto/issues/856)
// figure out how to test - cannot access imported statics on non-wasm
// targets
