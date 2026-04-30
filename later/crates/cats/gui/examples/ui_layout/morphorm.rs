// ANCHOR: example
use morphorm::*;

// Morphorm is a UI layout engine.
// In version 0.8, it requires implementing Cache, Tree, and Store traits.

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Entity(u32);

#[derive(Default)]
pub struct SimpleStore {
    pub width: std::collections::HashMap<Entity, Units>,
    pub height: std::collections::HashMap<Entity, Units>,
}

#[derive(Default)]
pub struct SimpleCache {
    pub rects: std::collections::HashMap<Entity, (f32, f32, f32, f32)>,
}

impl Cache for SimpleCache {
    type Node = Entity;
    fn width(&self, node: &Self::Node) -> f32 { self.rects.get(node).map(|r| r.2).unwrap_or(0.0) }
    fn height(&self, node: &Self::Node) -> f32 { self.rects.get(node).map(|r| r.3).unwrap_or(0.0) }
    fn posx(&self, node: &Self::Node) -> f32 { self.rects.get(node).map(|r| r.0).unwrap_or(0.0) }
    fn posy(&self, node: &Self::Node) -> f32 { self.rects.get(node).map(|r| r.1).unwrap_or(0.0) }
    fn set_bounds(&mut self, node: &Self::Node, x: f32, y: f32, w: f32, h: f32) {
        self.rects.insert(*node, (x, y, w, h));
    }
}

#[derive(Default)]
pub struct SimpleTree {
    pub children: std::collections::HashMap<Entity, Vec<Entity>>,
}

impl SimpleTree {
    pub fn add(&mut self, entity: Entity, parent: Option<Entity>) {
        if let Some(p) = parent {
            self.children.entry(p).or_default().push(entity);
        }
    }
}

pub struct ChildIter<'a> {
    pub iter: std::slice::Iter<'a, Entity>,
}

impl<'a> Iterator for ChildIter<'a> {
    type Item = &'a Entity;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl Node for Entity {
    type CacheKey = Entity;
    type Tree = SimpleTree;
    type Store = SimpleStore;
    type SubLayout<'a> = ();
    type ChildIter<'t> = ChildIter<'t>;

    fn key(&self) -> Self::CacheKey { *self }
    fn children<'t>(&self, tree: &'t Self::Tree) -> Self::ChildIter<'t> {
        static EMPTY: Vec<Entity> = Vec::new();
        ChildIter {
            iter: tree.children.get(self).unwrap_or(&EMPTY).iter()
        }
    }

    fn layout_type(&self, _store: &Self::Store) -> Option<LayoutType> { Some(LayoutType::Row) }
    fn width(&self, store: &Self::Store) -> Option<Units> { store.width.get(self).copied() }
    fn height(&self, store: &Self::Store) -> Option<Units> { store.height.get(self).copied() }

    // Required methods with default/stub implementations
    fn visible(&self, _: &Self::Store) -> bool { true }
    fn position_type(&self, _: &Self::Store) -> Option<PositionType> { Some(PositionType::Relative) }
    fn alignment(&self, _: &Self::Store) -> Option<Alignment> { None }
    fn left(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn right(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn top(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn bottom(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn content_size(&self, _: &Self::Store, _: &mut Self::SubLayout<'_>, _: Option<f32>, _: Option<f32>) -> Option<(f32, f32)> { None }
    fn padding_left(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn padding_right(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn padding_top(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn padding_bottom(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn vertical_gap(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn horizontal_gap(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn min_vertical_gap(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn min_horizontal_gap(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn max_vertical_gap(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn max_horizontal_gap(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn min_width(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn min_height(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn max_width(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn max_height(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn border_left(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn border_right(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn border_top(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn border_bottom(&self, _: &Self::Store) -> Option<Units> { Some(Units::Auto) }
    fn vertical_scroll(&self, _: &Self::Store) -> Option<f32> { Some(0.0) }
    fn horizontal_scroll(&self, _: &Self::Store) -> Option<f32> { Some(0.0) }
    fn grid_columns(&self, _: &Self::Store) -> Option<Vec<Units>> { None }
    fn grid_rows(&self, _: &Self::Store) -> Option<Vec<Units>> { None }
    fn column_start(&self, _: &Self::Store) -> Option<usize> { None }
    fn row_start(&self, _: &Self::Store) -> Option<usize> { None }
    fn column_span(&self, _: &Self::Store) -> Option<usize> { None }
    fn row_span(&self, _: &Self::Store) -> Option<usize> { None }
}

pub fn main() {
    let mut cache = SimpleCache::default();
    let mut tree = SimpleTree::default();
    let mut store = SimpleStore::default();

    let root = Entity(0);
    tree.add(root, None);
    store.width.insert(root, Units::Pixels(500.0));
    store.height.insert(root, Units::Pixels(400.0));

    let child1 = Entity(1);
    tree.add(child1, Some(root));
    store.width.insert(child1, Units::Pixels(100.0));
    store.height.insert(child1, Units::Pixels(100.0));

    let child2 = Entity(2);
    tree.add(child2, Some(root));
    store.width.insert(child2, Units::Pixels(200.0));
    store.height.insert(child2, Units::Pixels(100.0));

    root.layout(&mut cache, &tree, &store, &mut ());

    println!("Root bounds: ({}, {}, {}, {})", cache.posx(&root), cache.posy(&root), cache.width(&root), cache.height(&root));
    println!("Child 1 bounds: ({}, {}, {}, {})", cache.posx(&child1), cache.posy(&child1), cache.width(&child1), cache.height(&child1));
    println!("Child 2 bounds: ({}, {}, {}, {})", cache.posx(&child2), cache.posy(&child2), cache.width(&child2), cache.height(&child2));
}
// ANCHOR_END: example
