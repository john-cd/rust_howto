// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// use xilem::component::Component;
// use xilem::context::EventCx;
// use xilem::view::Adapt;
// use xilem::view::AnyView;
// use xilem::view::Button;
// use xilem::view::Label;
// use xilem::view::LinearLayout;
// use xilem::view::View;
// use xilem::view::ViewExt;

// // Requirements - see <https://github.com/linebender/xilem>
// // (Ubuntu): sudo apt-get install clang libwayland-dev libxkbcommon-x11-dev
// libvulkan-dev

// struct Counter {
//     count: i32,
// }

// // Create reusable UI elements with Component
// impl Component for Counter {
//     type Input = ();
//     type Output = ();

//     // Return the view hierarchy for the component.
//     // It uses `LinearLayout` to arrange the label and button vertically.
//     // `boxed()` is used to convert the specific view types into `AnyView`,
// which is     // necessary for the `LinearLayout`.
//     fn build(&mut self) -> AnyView {
//         LinearLayout::new(vec![
//             Label::new(format!("Count: {}", self.count)).boxed(),
//             Button::new("Increment", move |cx| cx.poke(())).boxed(),
//         ])
//         .boxed()
//     }

//     // The `event()` method is called when an event occurs within the
// component.     // This triggers an update to the component's state.
//     // In this case, when the button is clicked, `cx.poke(())` is called.
//     fn event(&mut self, _cx: &mut EventCx, event: &()) {
//         self.count += 1;
//     }

//     // The `update()` method is called when the component's input data
// changes.     // In this simple example, there's no external data being passed
// in, so the     // method is empty.
//     fn update(&mut self, _data: &Self::Input) {}
// }

// //  The app() function now returns the main view hierarchy for the
// application, // using the Adapt view to integrate the Counter component.
// fn app() -> impl View<AppModel> {
//     // The Adapt view is used to connect the component to the application's
// data     // model (AppModel). It takes two closures: one to extract the
//     // component's data from the model, and another to update the model with
//     // the component's new data.
//     Adapt::new(
//         |data: &AppModel| data.counter,
//         |data: &mut AppModel, value| data.counter = value,
//         Counter { count: 0 },
//     )
// }

// // The AppModel struct holds the application's state.
// struct AppModel {
//     counter: (),
// }

// fn main() -> Result<(), xilem::Error> {
//     // Start the application.
//     xilem::run(app(), AppModel { counter: 0 })
// }

pub fn main() {}
// // [finish; https://github.com/linebender/xilem ](https://github.com/john-cd/rust_howto/issues/795)
// // <https://raphlinus.github.io/rust/gui/2022/07/15/next-dozen-guis.html>
