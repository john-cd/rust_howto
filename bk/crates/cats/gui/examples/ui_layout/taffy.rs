// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example

// use taffy::prelude::*;
// use taffy::style::AlignItems;
// use taffy::style::JustifyContent;

// // Taffy is a flexible, high-performance, cross-platform UI layout library.
// // It currently implements the CSS Block, Flexbox and CSS Grid layout
// algorithms.

// fn main() {
//     // Create a new Taffy instance
//     let mut taffy = Taffy::new();

//     // Create some nodes
//     let root_node = taffy
//         .new_leaf(Style {
//             display: Display::Flex,
//             flex_direction: FlexDirection::Column,
//             align_items: AlignItems::Center,
//             justify_content: JustifyContent::Center,
//             size: Size {
//                 width: Dimension::Points(300.0),
//                 height: Dimension::Points(200.0),
//             },
//             ..Default::default()
//         })
//         .unwrap();

//     let child1 = taffy
//         .new_leaf(Style {
//             size: Size {
//                 width: Dimension::Points(100.0),
//                 height: Dimension::Points(50.0),
//             },
//             margin: Margin {
//                 bottom: Dimension::Points(10.0),
//                 ..Default::default()
//             },
//             ..Default::default()
//         })
//         .unwrap();

//     let child2 = taffy
//         .new_leaf(Style {
//             size: Size {
//                 width: Dimension::Points(150.0),
//                 height: Dimension::Points(75.0),
//             },
//             ..Default::default()
//         })
//         .unwrap();

//     // Add children to the root node
//     taffy.add_child(root_node, child1).unwrap();
//     taffy.add_child(root_node, child2).unwrap();

//     // Compute the layout
//     taffy
//         .compute_layout(root_node, Size {
//             width: AvailableSpace::Definite(300.0),
//             height: AvailableSpace::Definite(200.0),
//         })
//         .unwrap();

//     // Get the layout of the nodes
//     let root_layout = taffy.layout(root_node).unwrap();
//     let child1_layout = taffy.layout(child1).unwrap();
//     let child2_layout = taffy.layout(child2).unwrap();

//     // Print the layout information
//     println!("Root Layout: {:?}", root_layout);
//     println!("Child 1 Layout: {:?}", child1_layout);
//     println!("Child 2 Layout: {:?}", child2_layout);

//     // Example with FlexGrow
//     let mut taffy_flex_grow = Taffy::new();

//     let root_flex_grow = taffy_flex_grow
//         .new_leaf(Style {
//             display: Display::Flex,
//             size: Size {
//                 width: Dimension::Points(300.0),
//                 height: Dimension::Points(100.0),
//             },
//             ..Default::default()
//         })
//         .unwrap();

//     let child_flex_grow_1 = taffy_flex_grow
//         .new_leaf(Style {
//             flex_grow: 1.0,
//             ..Default::default()
//         })
//         .unwrap();

//     let child_flex_grow_2 = taffy_flex_grow
//         .new_leaf(Style {
//             flex_grow: 2.0,
//             ..Default::default()
//         })
//         .unwrap();

//     taffy_flex_grow
//         .add_child(root_flex_grow, child_flex_grow_1)
//         .unwrap();
//     taffy_flex_grow
//         .add_child(root_flex_grow, child_flex_grow_2)
//         .unwrap();

//     taffy_flex_grow
//         .compute_layout(root_flex_grow, Size {
//             width: AvailableSpace::Definite(300.0),
//             height: AvailableSpace::Definite(100.0),
//         })
//         .unwrap();

//     let child_flex_grow_1_layout =
//         taffy_flex_grow.layout(child_flex_grow_1).unwrap();
//     let child_flex_grow_2_layout =
//         taffy_flex_grow.layout(child_flex_grow_2).unwrap();

//     println!("Flex Grow Child 1 Layout: {:?}", child_flex_grow_1_layout);
//     println!("Flex Grow Child 2 Layout: {:?}", child_flex_grow_2_layout);
// }

pub fn main() {}
// // [WIP finish](https://github.com/john-cd/rust_howto/issues/788)
