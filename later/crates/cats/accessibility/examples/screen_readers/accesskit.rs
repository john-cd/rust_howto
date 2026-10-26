#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Describing a user interface to screen readers with `accesskit`.

use accesskit::Node;
use accesskit::NodeId;
use accesskit::Role;
use accesskit::TreeId;
use accesskit::TreeInfo;
use accesskit::TreeUpdate;

// AccessKit makes it easier to implement accessibility, for screen readers
// and other assistive technologies, in toolkits that render their own user
// interface elements. It provides a cross-platform, cross-language
// abstraction over accessibility APIs, so toolkit developers only have to
// implement accessibility once.

// In this example, we
// - create a simple accessibility tree with a root node and a button node.
/// Builds a small accessibility tree without creating a native window.
fn accessibility_tree() -> TreeUpdate {
    let root_id = NodeId(0);
    let save_button_id = NodeId(1);

    let mut root = Node::new(Role::Window);
    root.set_children(vec![save_button_id]);

    let mut save_button = Node::new(Role::Button);
    save_button.set_label("Save document");

    TreeUpdate {
        nodes: vec![(root_id, root), (save_button_id, save_button)],
        tree: Some(TreeInfo::new(root_id)),
        tree_id: TreeId::ROOT,
        focus: root_id,
    }
}

/// Prints the semantic nodes that a toolkit would send to its platform adapter.
fn main() {
    let update = accessibility_tree();
    println!("Accessibility tree contains {} nodes.", update.nodes.len());
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the tree exposes a button with a meaningful accessible
    /// name.
    #[test]
    fn tree_contains_named_save_button() {
        let update = accessibility_tree();
        let button = update
            .nodes
            .iter()
            .find(|(node_id, _)| *node_id == NodeId(1))
            .map(|(_, node)| node);

        assert!(button.is_some());
        if let Some(button) = button {
            assert_eq!(button.role(), Role::Button);
            assert_eq!(button.label(), Some("Save document"));
        }
    }
}
