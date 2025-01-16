// // ANCHOR: example
// // ANCHOR_END: example

// use dioxus::prelude::*;

// fn main() {
//     launch(App);
// }

// // App is our main component function. It uses the use_state hook to create
// and manage the count state. fn App(cx: Scope) -> Element {
//     let count = use_state(&cx, || 0);

//     // Render function where we define the structure and behavior of our
// component using Dioxus' rsx! syntax.     cx.render(rsx! {
//         div {
//             h1 { "Counter" }
//             button {
//                 onclick: move |_| count.set(*count - 1),
//                 "-",
//             }
//             span { "{count}" }
//             button {
//                 onclick: move |_| count.set(*count + 1),
//                 "+",
//             }
//         }
//     })
// }

fn main() {}
// // [P1](https://github.com/john-cd/rust_howto/issues/775)
