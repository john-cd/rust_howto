// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Computing a layout with `morphorm`.

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
    type Node = Entity;

    fn width(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.2)
    }

    fn height(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.3)
    }

    fn posx(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.0)
    }

    fn posy(&self, node: &Self::Node) -> f32 {
        self.rects.get(node).map_or(0.0, |r| r.1)
    }

    fn set_bounds(
        &mut self,
        node: &Self::Node,
        posx: f32,
        posy: f32,
        width: f32,
        height: f32,
    ) {
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
    type Item = &'a Entity;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Node for Entity {
    type CacheKey = Self;
    type ChildIter<'t> = ChildIter<'t>;
    type Store = SimpleStore;
    type SubLayout<'a> = ();
    type Tree = SimpleTree;

    fn key(&self) -> Self::CacheKey {
        *self
    }

    fn children<'t>(&self, tree: &'t Self::Tree) -> Self::ChildIter<'t> {
        /// Empty child list shared by nodes with no children.
        static EMPTY: Vec<Entity> = Vec::new();
        ChildIter {
            iter: tree.children.get(self).unwrap_or(&EMPTY).iter(),
        }
    }

    fn parent<'t>(&'t self, tree: &'t Self::Tree) -> Option<&'t Self> {
        tree.parents.get(self)
    }

    fn layout_type(&self, _store: &Self::Store) -> Option<LayoutType> {
        Some(LayoutType::Row)
    }

    fn width(&self, store: &Self::Store) -> Option<Units> {
        store.width.get(self).copied()
    }

    fn height(&self, store: &Self::Store) -> Option<Units> {
        store.height.get(self).copied()
    }

    // Required methods with default/stub implementations
    fn visible(&self, _: &Self::Store) -> bool {
        true
    }

    fn position_type(&self, _: &Self::Store) -> Option<PositionType> {
        Some(PositionType::Relative)
    }

    fn alignment(&self, _: &Self::Store) -> Option<Alignment> {
        None
    }

    fn left(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn right(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn top(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn bottom(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn content_size(
        &self,
        _: &Self::Store,
        _: &mut Self::SubLayout<'_>,
        _: Option<f32>,
        _: Option<f32>,
    ) -> Option<(f32, f32)> {
        None
    }

    fn padding_left(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn padding_right(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn padding_top(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn padding_bottom(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn vertical_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn horizontal_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn min_vertical_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn min_horizontal_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn max_vertical_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn max_horizontal_gap(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn min_width(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn min_height(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn max_width(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn max_height(&self, _: &Self::Store) -> Option<Units> {
        Some(Units::Auto)
    }

    fn grid_columns(&self, _: &Self::Store) -> Option<Vec<Units>> {
        None
    }

    fn grid_rows(&self, _: &Self::Store) -> Option<Vec<Units>> {
        None
    }

    fn column_start(&self, _: &Self::Store) -> Option<usize> {
        None
    }

    fn row_start(&self, _: &Self::Store) -> Option<usize> {
        None
    }

    fn column_span(&self, _: &Self::Store) -> Option<usize> {
        None
    }

    fn row_span(&self, _: &Self::Store) -> Option<usize> {
        None
    }
}

/// Runs the Morphorm layout example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a stretch child fills the parent space after a fixed
    /// sibling.
    #[test]
    fn stretch_child_uses_remaining_width() {
        let [_, _, stretch_child] = compute_layout();
        assert_eq!(stretch_child.0, 100.0);
        assert_eq!(stretch_child.2, 400.0);
    }

    /// Runs the deterministic Morphorm layout example entry point.
    #[test]
    fn test_morphorm_example_entry_point() {
        main();
    }
}
