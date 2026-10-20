#![allow(dead_code)]
// ANCHOR: example
use accesskit::Node;
use accesskit::NodeId;
use accesskit::Role;
use accesskit::Tree;
use accesskit::TreeId;
use accesskit::TreeUpdate;

// AccessKit makes it easier to implement accessibility, for screen readers
// and other assistive technologies, in toolkits that render their own user
// interface elements. It provides a cross-platform, cross-language
// abstraction over accessibility APIs, so toolkit developers only have to
// implement accessibility once.

// In this example, we
// - create a simple accessibility tree with a root node and a button node.
fn main() {
    let root_id = NodeId(0);
    let button_id = NodeId(1);

    // Create a basic tree structure
    let mut root = Node::new(Role::Window);
    root.set_children(vec![button_id]);

    let mut button = Node::new(Role::Button);
    // In accesskit 0.20, accessible names are set via `set_label`
    button.set_label("Click me");

    let tree = Tree::new(root_id);

    let update = TreeUpdate {
        nodes: vec![(root_id, root), (button_id, button)],
        tree: Some(tree),
        tree_id: TreeId::ROOT,
        focus: root_id,
    };

    assert_eq!(update.nodes.len(), 2);
    assert_eq!(update.tree.unwrap().root, root_id);
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
