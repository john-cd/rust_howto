// // ANCHOR: example
// COMING SOON
// // ANCHOR_END: example

// use fyrox::engine::Engine;
// use fyrox::engine::EngineInitParams;
// use fyrox::event_loop::EventLoop;
// use fyrox::gui::message::UiMessage;
// use fyrox::gui::message::UiMessageData;
// use fyrox::gui::message::WidgetMessage;
// use fyrox::gui::node::StubNode;
// use fyrox::gui::text::TextBuilder;
// use fyrox::gui::widget::WidgetBuilder;
// use fyrox::gui::window::WindowBuilder;
// use fyrox::gui::window::WindowTitle;
// use fyrox::scene::Scene;

// // This example creates a simple window with the title "Hello, Fyrox!" and a
// // text widget that displays "Welcome to Fyrox!".
// fn main() {
//     // Create an event loop
//     let event_loop = EventLoop::new();

//     // Initialize the engine
//     let mut engine = Engine::new(EngineInitParams {
//         ..Default::default()
//     })
//     .unwrap();

//     // Create a scene
//     let mut scene = Scene::new();

//     // Add a simple UI window
//     let window = WindowBuilder::new(WidgetBuilder::new())
//         .with_title(WindowTitle::text("Hello, Fyrox!"))
//         .build(&mut engine.user_interface.build_ctx());

//     // Add a text widget to the window
//     let _text = TextBuilder::new(WidgetBuilder::new())
//         .with_text("Welcome to Fyrox!")
//         .build(&mut engine.user_interface.build_ctx());

//     // Add the window to the scene
//     engine
//         .user_interface
//         .node_mut(window)
//         .link_to_parent(StubNode::new());

//     // Run the event loop
//     event_loop.run(move |event, _, control_flow| {
//         engine.update(&event);

//         // Handle UI events
//         while let Some(ui_message) = engine.user_interface.poll_message() {
//             if let UiMessage {
//                 data: UiMessageData::Widget(WidgetMessage::Close),
//                 ..
//             } = ui_message
//             {
//                 *control_flow = fyrox::event_loop::ControlFlow::Exit;
//             }
//         }
//     });
// }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/768)
