// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Dockable, tabbed panes with `egui_tiles`.
//!
//! A `Tree` stores the layout: containers (tabs, rows, columns, grids) and
//! panes. A `Behavior` says how to draw a pane and its tab.

use eframe::egui;
use egui_tiles::Behavior;
use egui_tiles::Tile;
use egui_tiles::TileId;
use egui_tiles::Tree;
use egui_tiles::UiResponse;

/// The content of a pane.
#[derive(Debug, PartialEq, Eq)]
struct Pane {
    /// The title shown on the tab.
    title: String,
}

impl Pane {
    /// Creates a pane.
    fn new(title: &str) -> Self {
        Self {
            title: title.to_owned(),
        }
    }
}

/// Draws the panes.
struct TreeBehavior;

impl Behavior<Pane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut egui::Ui,
        _tile_id: TileId,
        pane: &mut Pane,
    ) -> UiResponse {
        ui.heading(&pane.title);
        UiResponse::None
    }

    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        pane.title.as_str().into()
    }
}

/// Creates a tab container with two panes.
fn new_tree() -> Tree<Pane> {
    Tree::new_tabs("main", vec![Pane::new("Editor"), Pane::new("Console")])
}

/// Adds a pane next to the existing tabs and makes it the active one.
fn add_pane(tree: &mut Tree<Pane>, title: &str) -> Option<TileId> {
    let root = tree.root()?;
    let pane = tree.tiles.insert_pane(Pane::new(title));
    // Insert at the end (a large index is clamped) of the root container.
    tree.move_tile_to_container(pane, root, usize::MAX, true);
    tree.make_active(|id, _| id == pane);
    Some(pane)
}

/// Manipulates the layout without a window, then draws it for one frame.
fn main() {
    let mut tree = new_tree();
    // Two panes and the container that holds them.
    assert_eq!(tree.tiles.len(), 3);

    let Some(added) = add_pane(&mut tree, "Output") else {
        panic!("the tree has a root");
    };
    assert_eq!(tree.tiles.len(), 4);
    assert_eq!(tree.tiles.get_pane(&added), Some(&Pane::new("Output")));
    assert_eq!(tree.tiles.parent_of(added), tree.root());
    // The new tab is the only active pane.
    assert!(tree.active_tiles().contains(&added));

    // Removing a tile removes its children too.
    let removed = tree.remove_recursively(added);
    assert_eq!(removed.len(), 1);
    assert!(matches!(removed.first(), Some(Tile::Pane(_))));
    assert_eq!(tree.tiles.len(), 3);

    // Draw the tree. This needs an `egui::Context`, not a window.
    let ctx = egui::Context::default();
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    let _output = ctx.run_ui(input, |ui| {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            tree.ui(&mut TreeBehavior, ui);
        });
    });
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui tiles example through its entry point.
    #[test]
    fn test_egui_tiles_example_entry_point() {
        main();
    }

    #[test]
    fn empty_tree_has_no_root() {
        let mut tree: Tree<Pane> = Tree::empty("empty");
        assert!(tree.is_empty());
        assert_eq!(add_pane(&mut tree, "x"), None);
    }

    #[test]
    fn tab_titles_come_from_the_behavior() {
        let mut behavior = TreeBehavior;
        let title = behavior.tab_title_for_pane(&Pane::new("Editor"));
        assert_eq!(title.text(), "Editor");
    }
}
