// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Computing a layout with `morphorm`.
//!
//! A layout engine computes the size and position of every element from rules
//! such as "fill the width" or "stack vertically", so you do not place each
//! pixel by hand and the interface adapts to the window size.

use morphorm::*;

/// Builds a tree with fixed and stretch-sized children and computes its layout.
fn compute_layout() -> [(f32, f32, f32, f32); 3] {
    let mut cache = SimpleCache::default();
    let mut tree = SimpleTree::default();
    let mut store = SimpleStore::default();

    let root = Entity(0);
    tree.add(root, None);
    store.width.insert(root, Units::Pixels(500.0));
    store.height.insert(root, Units::Pixels(400.0));

    let fixed_child = Entity(1);
    tree.add(fixed_child, Some(root));
    store.width.insert(fixed_child, Units::Pixels(100.0));
    store.height.insert(fixed_child, Units::Pixels(100.0));

    let stretch_child = Entity(2);
    tree.add(stretch_child, Some(root));
    store.width.insert(stretch_child, Units::Stretch(1.0));
    store.height.insert(stretch_child, Units::Pixels(100.0));

    root.layout(&mut cache, &tree, &store, &mut ());
    let bounds = |node: &Entity| {
        (
            cache.posx(node),
            cache.posy(node),
            cache.width(node),
            cache.height(node),
        )
    };
    [bounds(&root), bounds(&fixed_child), bounds(&stretch_child)]
}

/// Prints the computed position and size of each node.
fn main() {
    let [root, fixed_child, stretch_child] = compute_layout();
    for (name, (x, y, width, height)) in [
        ("Root", root),
        ("Fixed child", fixed_child),
        ("Stretch child", stretch_child),
    ] {
        println!("{name}: x={x}, y={y}, width={width}, height={height}");
    }
}
// ANCHOR_END: example

/// A node of the layout tree.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(u32);

/// The sizes of the nodes.
#[derive(Default)]
pub struct SimpleStore {
    /// The width of each node.
    pub width: std::collections::HashMap<Entity, Units>,

    /// The height of each node.
    pub height: std::collections::HashMap<Entity, Units>,
}

/// The computed positions and sizes.
#[derive(Default)]
pub struct SimpleCache {
    /// The computed rectangle of each node.
    pub rects: std::collections::HashMap<Entity, (f32, f32, f32, f32)>,
}

impl Cache for SimpleCache {
    /// Alias for `Entity` used to represent node.
    type Node = Entity;

    /// Handles the width behavior for this component.
    fn width(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.2)
    }

    /// Handles the height behavior for this component.
    fn height(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.3)
    }

    /// Handles the posx behavior for this component.
    fn posx(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.0)
    }

    /// Handles the posy behavior for this component.
    fn posy(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.1)
    }

    /// Changes the bounds held by the simple cache; subsequent operations use the updated value.
    fn set_bounds(&mut self, node: &Self::Node, posx: f32, posy: f32, width: f32, height: f32) {
        self.rects.insert(*node, (posx, posy, width, height));
    }
}

/// The tree of nodes.
#[derive(Default)]
pub struct SimpleTree {
    /// The children of each node.
    pub children: std::collections::HashMap<Entity, Vec<Entity>>,

    /// The parent of each node.
    pub parents: std::collections::HashMap<Entity, Entity>,
}

impl SimpleTree {
    /// Adds `entity` to the tree, as a child of `parent` if there is one.
    pub fn add(&mut self, entity: Entity, parent: Option<Entity>) {
        if let Some(p) = parent {
            self.parents.insert(entity, p);
            self.children.entry(p).or_default().push(entity);
        }
    }
}

/// An iterator over the children of a node.
pub struct ChildIter<'a> {
    /// The underlying iterator.
    pub iter: std::slice::Iter<'a, Entity>,
}

impl<'a> Iterator for ChildIter<'a> {
    /// Alias for `&'a Entity` used to represent item.
    type Item = &'a Entity;

    /// Returns the next value produced by this iterator or protocol stream.
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Node for Entity {
    /// Alias for `Self` used to represent cache key.
    type CacheKey = Self;
    /// Alias for `ChildIter<'t>` used to represent child iter.
    type ChildIter<'t> = ChildIter<'t>;
    /// Alias for `SimpleStore` used to represent store.
    type Store = SimpleStore;
    /// Alias for `()` used to represent sub layout.
    type SubLayout<'a> = ();
    /// Alias for `SimpleTree` used to represent tree.
    type Tree = SimpleTree;

    /// Handles the key behavior for this component.
    fn key(&self) -> Self::CacheKey {
        *self
    }

    /// Handles the children behavior for this component.
    fn children<'t>(&self, tree: &'t Self::Tree) -> Self::ChildIter<'t> {
        /// Empty child list shared by nodes with no children.
        static EMPTY: Vec<Entity> = Vec::new();
        ChildIter {
            iter: tree.children.get(self).unwrap_or(&EMPTY).iter(),
        }
    }

    /// Handles the parent behavior for this component.
    fn parent<'t>(&'t self, tree: &'t Self::Tree) -> Option<&'t Self> {
        tree.parents.get(self)
    }

    /// Handles the layout type behavior for this component.
    fn layout_type(&self, _store: &Self::Store) -> Option<LayoutType> {
        Some(LayoutType::Row)
    }

    /// Handles the width behavior for this component.
    fn width(&self, store: &Self::Store) -> Option<Units> {
        store.width.get(self).copied()
    }

    /// Handles the height behavior for this component.
    fn height(&self, store: &Self::Store) -> Option<Units> {
        store.height.get(self).copied()
    }

    // Required methods with default/stub implementations

    fn visible(&self, _: &Self::Store) -> bool {
        true
    }

    /// Handles the position type behavior for this component.
    fn position_type(&self, _: &Self::Store) -> Option<PositionType> {
        Some(PositionType::Relative)
    }

    /// Handles the alignment behavior for this component.
    fn alignment(&self, _: &Self::Store) -> Option<Alignment> {
        None
    }

    /// Handles the left behavior for this component.
    fn left(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the right behavior for this component.
    fn right(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the top behavior for this component.
    fn top(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the bottom behavior for this component.
    fn bottom(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the content size behavior for this component.
    fn content_size(
        &self,
        _: &Self::Store,
        (): &mut Self::SubLayout<'_>,
        _: Option<f32>,
        _: Option<f32>,
    ) -> Option<(f32, f32)> {
        None
    }

    /// Handles the padding left behavior for this component.
    fn padding_left(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the padding right behavior for this component.
    fn padding_right(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the padding top behavior for this component.
    fn padding_top(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the padding bottom behavior for this component.
    fn padding_bottom(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the vertical gap behavior for this component.
    fn vertical_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the horizontal gap behavior for this component.
    fn horizontal_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the min vertical gap behavior for this component.
    fn min_vertical_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the min horizontal gap behavior for this component.
    fn min_horizontal_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the max vertical gap behavior for this component.
    fn max_vertical_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the max horizontal gap behavior for this component.
    fn max_horizontal_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Pixels(0.0))
    }

    /// Handles the min width behavior for this component.
    fn min_width(&self, _: &Self::Store) -> Option<Units> {
        None
    }

    /// Handles the min height behavior for this component.
    fn min_height(&self, _: &Self::Store) -> Option<Units> {
        None
    }

    /// Handles the max width behavior for this component.
    fn max_width(&self, _: &Self::Store) -> Option<Units> {
        None
    }

    /// Handles the max height behavior for this component.
    fn max_height(&self, _: &Self::Store) -> Option<Units> {
        None
    }

    /// Handles the grid columns behavior for this component.
    fn grid_columns(&self, _: &Self::Store) -> Option<Vec<Units>> {
        None
    }

    /// Handles the grid rows behavior for this component.
    fn grid_rows(&self, _: &Self::Store) -> Option<Vec<Units>> {
        None
    }

    /// Handles the column start behavior for this component.
    fn column_start(&self, _: &Self::Store) -> Option<usize> {
        None
    }

    /// Handles the row start behavior for this component.
    fn row_start(&self, _: &Self::Store) -> Option<usize> {
        None
    }

    /// Handles the column span behavior for this component.
    fn column_span(&self, _: &Self::Store) -> Option<usize> {
        None
    }

    /// Handles the row span behavior for this component.
    fn row_span(&self, _: &Self::Store) -> Option<usize> {
        None
    }
}

/// Runs the Morphorm layout example.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
#[cfg(test)]
mod tests {
    use super::*;

    /// Computes each node's position and size from the layout tree.
    #[test]
    fn computes_positions_and_sizes_for_each_node() {
        let [root, fixed_child, stretch_child] = compute_layout();
        assert_eq!(root, (0.0, 0.0, 500.0, 400.0));
        assert_eq!(fixed_child, (0.0, 0.0, 100.0, 100.0));
        assert_eq!(stretch_child, (100.0, 0.0, 400.0, 100.0));
    }

    /// Runs the deterministic Morphorm layout example entry point.
    #[test]
    fn test_morphorm_example_entry_point() {
        main();
    }
}
