// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Computes and inspects a flexbox layout with taffy.

use taffy::prelude::*;

/// Computes the x and y positions and sizes of two flex-row children.
fn compute_child_layouts() -> anyhow::Result<[(f32, f32, f32, f32); 2]> {
    let mut tree: TaffyTree<()> = TaffyTree::new();
    let fixed_child = tree.new_leaf(Style {
        size: Size {
            width: length(100.0),
            height: length(50.0),
        },
        ..Default::default()
    })?;
    let flexible_child = tree.new_leaf(Style {
        size: Size {
            width: auto(),
            height: length(50.0),
        },
        flex_grow: 1.0,
        ..Default::default()
    })?;
    let root = tree.new_with_children(
        Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            size: Size {
                width: length(400.0),
                height: length(80.0),
            },
            ..Default::default()
        },
        &[fixed_child, flexible_child],
    )?;

    tree.compute_layout(root, Size::MAX_CONTENT)?;
    let fixed_layout = tree.layout(fixed_child)?;
    let flexible_layout = tree.layout(flexible_child)?;
    Ok([
        (
            fixed_layout.location.x,
            fixed_layout.location.y,
            fixed_layout.size.width,
            fixed_layout.size.height,
        ),
        (
            flexible_layout.location.x,
            flexible_layout.location.y,
            flexible_layout.size.width,
            flexible_layout.size.height,
        ),
    ])
}

/// Prints each child's computed position and size.
fn main() -> anyhow::Result<()> {
    for (index, (x, y, width, height)) in
        compute_child_layouts()?.into_iter().enumerate()
    {
        println!(
            "Child {}: x = {x}, y = {y}, width = {width}, height = {height}",
            index + 1
        );
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the taffy layout example.
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that both flex-row children have the expected positions and
    /// sizes.
    #[test]
    fn flex_row_computes_positions_and_sizes() -> anyhow::Result<()> {
        let [
            (fixed_x, fixed_y, fixed_width, fixed_height),
            (flexible_x, flexible_y, flexible_width, flexible_height),
        ] = compute_child_layouts()?;
        assert_eq!(
            (fixed_x, fixed_y, fixed_width, fixed_height),
            (0.0, 0.0, 100.0, 50.0)
        );
        assert_eq!(
            (flexible_x, flexible_y, flexible_width, flexible_height),
            (100.0, 0.0, 300.0, 50.0)
        );
        Ok(())
    }

    /// Runs the deterministic layout example entry point.
    #[test]
    fn test_taffy_example_entry_point() -> anyhow::Result<()> {
        main()
    }
}
