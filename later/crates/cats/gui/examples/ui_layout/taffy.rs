// ANCHOR: example
use taffy::prelude::*;

// Taffy is a flexible, high-performance UI layout library.
// It implements CSS Block, Flexbox, and CSS Grid layout algorithms.

pub fn main() {
    // Create a new TaffyTree instance
    let mut tree: TaffyTree<()> = TaffyTree::new();

    // Create leaf nodes with fixed dimensions
    let child1 = tree.new_leaf(
        Style {
            size: Size { width: length(100.0), height: length(50.0) },
            ..Default::default()
        },
    ).unwrap();

    let child2 = tree.new_leaf(
        Style {
            size: Size { width: length(150.0), height: length(75.0) },
            ..Default::default()
        },
    ).unwrap();

    // Create a root container with Flexbox layout
    let root = tree.new_with_children(
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_items: Some(AlignItems::Center),
            justify_content: Some(JustifyContent::Center),
            size: Size { width: length(300.0), height: length(200.0) },
            ..Default::default()
        },
        &[child1, child2],
    ).unwrap();

    // Compute the layout
    tree.compute_layout(root, Size::MAX_CONTENT).unwrap();

    // Get the computed layout of the nodes
    let root_layout = tree.layout(root).unwrap();
    let child1_layout = tree.layout(child1).unwrap();
    let child2_layout = tree.layout(child2).unwrap();

    println!("Root size: {}x{}", root_layout.size.width, root_layout.size.height);
    println!("Child 1 position: ({}, {})", child1_layout.location.x, child1_layout.location.y);
    println!("Child 2 position: ({}, {})", child2_layout.location.x, child2_layout.location.y);
}
// ANCHOR_END: example
