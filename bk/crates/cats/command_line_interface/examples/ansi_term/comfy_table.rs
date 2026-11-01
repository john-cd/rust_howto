// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Print a table whose columns adapt to their content.
//!
//! Tables make command-line output readable. This crate measures the content
//! and wraps or sizes columns to fit the terminal width.

use comfy_table::CellAlignment;
use comfy_table::ContentArrangement;
use comfy_table::Table;
use comfy_table::presets::ASCII_FULL;

/// Builds a table of crates, with a right-aligned numeric column.
fn build_table() -> Table {
    let mut table = Table::new();
    table
        // A plain ASCII preset renders the same everywhere. Use
        // `presets::UTF8_FULL` for box-drawing characters.
        .load_style(ASCII_FULL)
        // Wrap long cells to the table width instead of overflowing. Without
        // `set_width`, the width of the attached terminal is used.
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(40)
        .set_header(vec!["Crate", "Purpose", "Count"])
        .add_row(vec!["comfy-table", "Pretty tables for the terminal", "12"])
        .add_row(vec!["tabled", "Tables from structs", "345"]);
    // Numbers read better right-aligned.
    if let Some(column) = table.column_mut(2) {
        column.set_cell_alignment(CellAlignment::Right);
    }
    table
}

/// Prints the table.
pub fn run() {
    println!("{}", build_table());
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_line_fits_the_width() {
        let rendered = build_table().to_string();
        assert!(rendered.lines().all(|line| line.chars().count() <= 40));
        assert!(rendered.contains("comfy-table"));
    }

    #[test]
    fn run_succeeds() {
        run();
    }
}
