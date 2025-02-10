// // ANCHOR: example
// // ANCHOR_END: example

// use accesskit::Action;
// use accesskit::ActionRequest;
// use accesskit::Live;
// use accesskit::Node;
// use accesskit::NodeId;
// use accesskit::Rect;
// use accesskit::Role;
// use accesskit::Tree;
// use accesskit::TreeUpdate;
// use accesskit_winit::WindowEvent;
// //use winit::event::WindowEvent;
// use accesskit_winit::Event;
// use winit::event_loop::ControlFlow;

// // Uses accesskit_winit to connect AccessKit to the Winit windowing system.

// // AccessKit makes it easier to implement accessibility, for screen readers
// and // other assistive technologies, in toolkits that render their own user
// // interface elements. It provides a cross-platform, cross-language
// abstraction // over accessibility APIs, so toolkit developers only have to
// implement // accessibility once.

// // In this example, we
// // - create a simple accessibility tree with a root node and a button node.
// // - implement an `ActionHandler` to handle actions on the nodes.
// // - print the tree structure and simulate an action on the button node.

// fn main() {
//     let event_loop = winit::event_loop::EventLoop::new();
//     let window = WindowBuilder::new().build(&event_loop).unwrap();

//     let mut adapter = accesskit_winit::Adapter::new(&window);

//     // Build a simple AccessKit tree with a button and static text
//     // The heart of AccessKit is a data schema that defines all the data
//     // required to render an accessible UI for screen readers and other
//     // assistive technologies. The schema represents a tree structure, in
// which     // each node is either a single UI element or an element cluster
// such as a     // window or document. Each node has an integer ID, a role
// (e.g. button,     // label, or text input), and a variety of optional
// attributes. The schema     // also defines actions that can be requested by
// assistive technologies,     // such as moving the keyboard focus, invoking a
// button, or selecting text.     let mut tree = {
//         let mut root = NodeBuilder::new(Role::Application);
//         root.set_children(vec![
//             {
//                 let mut button = NodeBuilder::new(Role::Button);
//                 button.set_name("Click me!");
//                 button.set_state(State::default()); // Start with no state
//                 button.build()
//             },
//             {
//                 let mut static_text = NodeBuilder::new(Role::StaticText);
//                 static_text.set_name("Some static text");
//                 static_text.build()
//             },
//         ]);
//         Tree::new(root.build())
//     };

//     event_loop.run(move |event, _, control_flow| {
//         *control_flow = ControlFlow::Poll;

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 ..
//             } => *control_flow = ControlFlow::Exit,

//             // Handles the MouseInput event to detect button clicks.
//             Event::WindowEvent {
//                 event: WindowEvent::MouseInput { state, button, .. },
//                 ..
//             } => {
//                 if button == winit::event::MouseButton::Left {
//                     let node_id = tree.root().children()[0].id(); // ID of
// the button

//                     // Set and update the state of a node (e.g., PRESSED,
//                     // FOCUSED).
//                     if state == winit::event::ElementState::Pressed {
//                         // Update the state and perform the action
//                         let mut update = TreeUpdate::default();
//                         let mut button_node =
//                             tree.get(node_id).unwrap().to_owned();
//                         button_node.set_state(State::FOCUSED |
// State::PRESSED);                         update.set_node(button_node);
//                         tree.update(update);

//                         println!("Button clicked!");
//                         // You would usually perform the button's action
// here.                     } else {
//                         // Released
//                         // Use TreeUpdate to efficiently update the tree.
// This                         // is important for performance, especially with
// larger                         // trees. Only the changed node is updated.
//                         let mut update = TreeUpdate::default();
//                         let mut button_node =
//                             tree.get(node_id).unwrap().to_owned();
//                         button_node.set_state(State::FOCUSED); // Back to
// focused after click                         update.set_node(button_node);
//                         tree.update(update);
//                     }
//                     // Tell accesskit_winit to send the updated tree to the
//                     // accessibility API.
//                     adapter.update(&tree);
//                 }
//             }

//             _ => (),
//         }
//     });
// }

// // fn main2() {
// //     // Create a basic tree structure
// //     let root = Node {
// //         role: Role::RootWebArea,
// //         ..Default::default()
// //     };

// //     let button = Node {
// //         role: Role::Button,
// //         name: Some("Click me".into()),
// //         ..Default::default()
// //     };

// //     let tree = Tree::new(root);
// //     let button_id = tree.append_child(NodeId::new(1), button);

// //     // Create an action handler
// //     struct MyActionHandler;
// //     impl ActionHandler for MyActionHandler {
// //         fn do_action(&self, node_id: NodeId, action: &str) {
// //             println!("Action: {} on Node: {:?}", action, node_id);
// //         }
// //     }

// //     let handler = MyActionHandler;

// //     // Print the tree structure and handle actions
// //     println!("Tree structure: {:?}", tree);
// //     handler.do_action(button_id, "activate");
// // }

// #[test]
// fn test() {
//     main();
// }
// // [P2](https://github.com/john-cd/rust_howto/issues/676)
// // https://accesskit.dev/
