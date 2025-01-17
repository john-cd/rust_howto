// // ANCHOR: example
// use morphorm::*;
// // use morphorm_ecs::*;

// fn main() {
//     // The basic building blocks of a Morphorm layout are nodes. Each node
//     // represents an element in the UI.
//     // Nodes are organized in a tree structure, where each node can have
// parent     // and child nodes.
//     // Create a root node:
//     let mut world = World::default();

//     let root = world.add(None);
//     //let mut root = morphorm::Node::new();
//     // Morphorm uses units to specify sizes and positions. These include:
//     // - Absolute Pixel values,
//     // - Percentage: Percentage of the parent's size,
//     // - Stretch: Distributes remaining space.
//     root.set_width(Units::Pixels(500.0));
//     root.set_height(Units::Pixels(400.0));
//     root.set_layout_type(LayoutType::Row); // Arrange children in a row

//     // Create a child node (red box)
//     let mut red_box = morphorm::Node::new();
//     red_box.set_width(Units::Pixels(100.0));
//     red_box.set_height(Units::Pixels(100.0));
//     red_box.set_background_color(morphorm::Color::rgb(255, 0, 0)); // Red
//     root.add(&mut red_box);

//     // Create another child node (blue box)
//     let mut blue_box = morphorm::Node::new();
//     blue_box.set_width(Units::Pixels(150.0));
//     blue_box.set_height(Units::Pixels(100.0));
//     blue_box.set_background_color(morphorm::Color::rgb(0, 0, 255)); // Blue
//     root.add(&mut blue_box);

//     // Create a third child node (green box) with flex grow
//     let mut green_box = morphorm::Node::new();
//     green_box.set_height(Units::Pixels(100.0));
//     green_box.set_flex_grow(1.0); // Takes up remaining space
//     green_box.set_background_color(morphorm::Color::rgb(0, 255, 0)); // Green
//     root.add(&mut green_box);

//     // Calculate the layout
//     root.layout(500.0, 400.0); // Provide root width and height

//     // Print the layout results
//     println!("Root: {:?}", root.get_bounds());
//     println!("Red Box: {:?}", red_box.get_bounds());
//     println!("Blue Box: {:?}", blue_box.get_bounds());
//     println!("Green Box: {:?}", green_box.get_bounds());

//     // Example with absolute positioning
//     let mut root_absolute = morphorm::Node::new();
//     root_absolute.set_width(Units::Pixels(500.0));
//     root_absolute.set_height(Units::Pixels(400.0));

//     let mut absolute_box = morphorm::Node::new();
//     absolute_box.set_position_type(PositionType::SelfDirected);
//     absolute_box.set_left(Units::Pixels(50.0));
//     absolute_box.set_top(Units::Pixels(50.0));
//     absolute_box.set_width(Units::Pixels(100.0));
//     absolute_box.set_height(Units::Pixels(100.0));
//     absolute_box.set_background_color(morphorm::Color::rgb(255, 255, 0)); //
// Yellow

//     root_absolute.add(&mut absolute_box);
//     root_absolute.layout(500.0, 400.0);
//     println!("\nRoot (Absolute): {:?}", root_absolute.get_bounds());
//     println!("Absolute Box: {:?}", absolute_box.get_bounds());

//     // Example with column layout and margin
//     let mut root_column = morphorm::Node::new();
//     root_column.set_width(Units::Pixels(300.0));
//     root_column.set_height(Units::Pixels(300.0));
//     root_column.set_layout_type(LayoutType::Column);

//     let mut margin_box = morphorm::Node::new();
//     margin_box.set_width(Units::Pixels(100.0));
//     margin_box.set_height(Units::Pixels(50.0));
//     margin_box.set_margin_left(Units::Pixels(20.0));
//     margin_box.set_margin_top(Units::Pixels(10.0));
//     margin_box.set_background_color(morphorm::Color::rgb(255, 165, 0)); //
// Orange

//     root_column.add(&mut margin_box);
//     root_column.layout(300.0, 300.0);

//     println!(
//         "\nRoot (Column with Margin): {:?}",
//         root_column.get_bounds()
//     );
//     println!("Margin Box: {:?}", margin_box.get_bounds());
// }
// // ANCHOR_END: example

fn main() {}
// // [P2](https://github.com/john-cd/rust_howto/issues/782)
