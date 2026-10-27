// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Computing a flexbox layout with `taffy`.

use taffy::prelude::*;

// Taffy is a flexible, high-performance UI layout library.
// It implements CSS Block, Flexbox, and CSS Grid layout algorithms.

/// Builds a tree of nodes and computes its layout.
pub fn main() {
    // Create a new TaffyTree instance
    let mut tree: TaffyTree<()> = TaffyTree::new();

    // Create leaf nodes with fixed dimensions
    let child1 = tree
        .new_leaf(Style {
            size: Size {
                width: length(100.0_f32),
                height: length(50.0_f32),
            },
            ..Default::default()
        })
        .expect("the taffy example invariant should hold");

    let child2 = tree
        .new_leaf(Style {
            size: Size {
                width: length(150.0_f32),
                height: length(75.0_f32),
            },
            ..Default::default()
        })
        .expect("the taffy example invariant should hold");

    // Create a root container with Flexbox layout
    let root = tree
        .new_with_children(
            Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: Some(AlignItems::Center),
                justify_content: Some(JustifyContent::Center),
                size: Size {
                    width: length(300.0_f32),
                    height: length(200.0_f32),
                },
                ..Default::default()
            },
            &[child1, child2],
        )
        .expect("the taffy example invariant should hold");

    // Compute the layout
    tree.compute_layout(root, Size::MAX_CONTENT)
        .expect("the taffy example invariant should hold");

    // Get the computed layout of the nodes
    let root_layout = tree
        .layout(root)
        .expect("the taffy example invariant should hold");
    let child1_layout = tree
        .layout(child1)
        .expect("the taffy example invariant should hold");
    let child2_layout = tree
        .layout(child2)
        .expect("the taffy example invariant should hold");

    println!(
        "Root size: {}x{}",
        root_layout.size.width, root_layout.size.height
    );
    println!(
        "Child 1 position: ({}, {})",
        child1_layout.location.x, child1_layout.location.y
    );
    println!(
        "Child 2 position: ({}, {})",
        child2_layout.location.x, child2_layout.location.y
    );
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires an interactive example runtime"]
    fn test_main() {
        main();
    }
}
