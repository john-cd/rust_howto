// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Toast notifications with `egui-notify` and icons with `egui-phosphor`.

use std::time::Duration;

use eframe_036::egui;
use egui_notify::Toasts;

/// Registers the Phosphor icon font as a fallback of the default font.
///
/// Call it once, when the application starts.
fn install_icons(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
    ctx.set_fonts(fonts);
}

/// The application state.
struct App {
    /// The pending notifications.
    toasts: Toasts,
    /// The items of the list.
    items: Vec<String>,
}

impl App {
    /// Creates an app with two items.
    fn new() -> Self {
        Self {
            toasts: Toasts::default(),
            items: vec!["one".to_owned(), "two".to_owned()],
        }
    }

    /// Draws the list. The icons are characters of a font, so they are part
    /// of the text of the buttons.
    fn show(&mut self, ui: &mut egui::Ui) {
        if ui
            .button(format!("{} Add", egui_phosphor::regular::PLUS))
            .clicked()
        {
            self.add_item();
        }
        if ui
            .button(format!("{} Delete last", egui_phosphor::regular::TRASH))
            .clicked()
        {
            self.delete_last();
        }
        for item in &self.items {
            ui.label(item);
        }
        // Draw the toasts last, on top of the rest.
        self.toasts.show(ui.ctx());
    }

    /// Adds an item and tells the user.
    fn add_item(&mut self) {
        self.items.push(format!("item {}", self.items.len() + 1));
        self.toasts
            .success("Item added")
            .duration(Some(Duration::from_secs(3)));
    }

    /// Deletes the last item, or warns when there is none.
    fn delete_last(&mut self) {
        if self.items.pop().is_some() {
            self.toasts.info("Item deleted");
        } else {
            self.toasts.warning("Nothing to delete");
        }
    }
}

/// Exercises the app without a window.
fn run_example() {
    let ctx = egui::Context::default();
    install_icons(&ctx);

    let mut app = App::new();
    assert!(app.toasts.is_empty());
    app.add_item();
    app.delete_last();
    app.delete_last();
    app.delete_last();
    assert_eq!(app.toasts.len(), 4);
    assert!(app.items.is_empty());

    // Draw one frame: the toasts are painted on top.
    let screen =
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0));
    let input = egui::RawInput {
        screen_rect: Some(screen),
        ..Default::default()
    };
    let mut output = ctx.run_ui(input, |ui| {
        egui::CentralPanel::default().show(ui, |ui| app.show(ui));
    });
    // A renderer would upload the textures; drop them on purpose.
    output.textures_delta.clear();
}
fn main() {
    run_example();
}
// ANCHOR_END: example

pub fn run() {
    run_example();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the egui toasts icons example through its entry point.
    #[test]
    fn test_egui_toasts_icons_example_entry_point() {
        main();
    }

    #[test]
    fn icon_font_is_registered() {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(
            &mut fonts,
            egui_phosphor::Variant::Regular,
        );
        assert!(fonts.font_data.contains_key("phosphor"));
    }
}
