// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Describes a small interface to screen readers with AccessKit.

use accesskit::Node;
use accesskit::NodeId;
use accesskit::Role;
use accesskit::TreeId;
use accesskit::TreeInfo;
use accesskit::TreeUpdate;

/// Builds a window, a button, and a text label for a platform adapter.
fn accessibility_tree() -> TreeUpdate {
    let window_id = NodeId(0);
    let button_id = NodeId(1);
    let label_id = NodeId(2);

    let mut window = Node::new(Role::Window);
    window.set_children(vec![button_id, label_id]);

    let mut button = Node::new(Role::Button);
    button.set_label("Save document");

    let mut label = Node::new(Role::Label);
    label.set_value("Changes are ready to save");

    TreeUpdate {
        nodes: vec![
            (window_id, window),
            (button_id, button),
            (label_id, label),
        ],
        tree: Some(TreeInfo::new(window_id)),
        tree_id: TreeId::ROOT,
        focus: button_id,
    }
}

/// Prints the number of semantic nodes prepared for an adapter.
fn main() {
    let update = accessibility_tree();
    println!("Accessibility tree contains {} nodes.", update.nodes.len());
}
// ANCHOR_END: example

/// Runs the headless AccessKit example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies node roles, names, text, and keyboard focus in the tree.
    #[test]
    fn tree_exposes_button_and_label_with_button_focus() {
        let update = accessibility_tree();
        let window_id = NodeId(0);
        let button_id = NodeId(1);
        let label_id = NodeId(2);

        let window = find_node(&update, window_id);
        assert_eq!(window.role(), Role::Window);
        assert_eq!(window.children(), &[button_id, label_id]);

        let button = find_node(&update, button_id);
        assert_eq!(button.role(), Role::Button);
        assert_eq!(button.label(), Some("Save document"));

        let label = find_node(&update, label_id);
        assert_eq!(label.role(), Role::Label);
        assert_eq!(label.value(), Some("Changes are ready to save"));
        assert_eq!(update.focus, button_id);
    }

    /// Finds a node in an accessibility tree and reports a missing fixture
    /// node.
    fn find_node(update: &TreeUpdate, id: NodeId) -> &Node {
        update
            .nodes
            .iter()
            .find(|(node_id, _)| *node_id == id)
            .map(|(_, node)| node)
            .expect("the accessibility tree contains each requested node")
    }
}
