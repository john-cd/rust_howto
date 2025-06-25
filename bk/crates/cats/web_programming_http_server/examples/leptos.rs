// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This is a simple counter example using the Leptos framework.
// //!
// //! It demonstrates basic concepts like reactive signals, derived signals,
// //! conditional rendering, and dynamic list creation.

// use leptos::mount::mount_to_body;
// use leptos::prelude::*;

// #[component] // Annotate a function, so it can be used as a component.
// fn App() -> impl IntoView {
//     // Reactive signal for the counter.
//     // Make the UI automatically update when the count changes.
//     let (count, set_count) = signal(0);

//     // Derived signal for even/odd status.
//     // It recalculates automatically based on the count signal,
//     // avoiding manual updates.
//     let is_even = move || count.get() % 2 == 0;
//     // The view macro defines the UI structure.
//     view! {
//             <main>
//                 <h1>"Leptos Counter Example"</h1>

//                 <p>"Count: " {count}</p>

//                 <button on:click=move |_| set_count.update(|n| *n += 1)>
//                     "+"
//                 </button>
//                 <button on:click=move |_| set_count.update(|n| *n -= 1)>
//                     "-"
//                 </button>

//                 <p>"The count is " {move || if is_even() { "even" } else {
// "odd" }}</p>                 // Example of showing/hiding content based on a
// signal,                 // using conditional rendering.
//                 // Note the use of a CSS class (highlight) to style elements.
//                 { move || if count.get() > 5 {
//                     view! { <p class="highlight">"Count is greater than
// 5!"</p> }.into_view() }                     else
//                     {
//                     view! { <p class="highlight"></p> }.into_view()
//                     }
//                 }
//                 // Dynamically create a list using map and collect_view.
//                 <h2>"List of Numbers"</h2>
//                 <ul>
//                     {move || count.map(|n| view! { <li>{n}</li>
// }).collect_view()}                 </ul>
//             </main>
//         }
// }

fn main() {
    //     // Mount the app to the HTML body.
    //     mount_to_body(App)
}

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/867)
// // <https://leptos.dev/>
// // <https://github.com/leptos-rs/awesome-leptos>
