// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use floem::Application;
// use floem::reactive::Signal;
// use floem::reactive::create_signal;
// use floem::style::Style;
// use floem::view::View;
// use floem::views::button;
// use floem::views::container;
// use floem::views::label;
// use floem::views::stack;

// fn app_view() -> impl View {
//     let (counter, set_counter) = create_signal(0);

//     stack((
//         label(move || format!("Counter: {}", counter.get()))
//             .style(|s| s.padding(10.0).font_size(20.0)),
//         button(move || "Increment".to_string())
//             .on_click(move |_| {
//                 set_counter.update(|counter| *counter += 1);
//             })
//             .style(|s| s.padding(10.0)),
//         button(move || "Decrement".to_string())
//             .on_click(move |_| {
//                 set_counter.update(|counter| *counter -= 1);
//             })
//             .style(|s| s.padding(10.0)),
//         // Example of conditional rendering
//         conditional_view(counter).style(|s| s.padding(10.0)),
//     ))
//     .style(|s| {
//         s.size_pct(100.0, 100.0)
//             .flex_col()
//             .align_items_center()
//             .justify_content_center()
//     })
// }

// fn conditional_view(counter: Signal<i32>) -> impl View {
//     container(label(move || {
//         if counter.get() % 2 == 0 {
//             "Counter is even!".to_string()
//         } else {
//             "Counter is odd!".to_string()
//         }
//     }))
//     .style(move |s| {
//         if counter.get() % 2 == 0 {
//             s.background(floem::style::Color::LIGHT_GREEN)
//         } else {
//             s.background(floem::style::Color::LIGHT_YELLOW)
//         }
//     })
// }

// fn main() {
//     Application::new().run(|_| app_view())
// }

pub fn main() {}
// // [P1](https://github.com/john-cd/rust_howto/issues/778)
