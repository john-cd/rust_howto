#![allow(dead_code)]

// TODO Reviewed ??-??-2026
// ANCHOR: example
//! Placeholder for a 2D rendering example with `skia-safe`.

// skia-safe example (commented out due to native build requirements in the
// workspace)
// use skia_safe::{Canvas, Color, Paint, Rect, Surface};
//
// pub fn main() {
// let mut surface = Surface::new_raster_n32_premul((600, 400)).expect("Failed
// to create Skia surface"); let canvas = surface.canvas();
// canvas.clear(Color::WHITE);
// }
/// Prints a message: the example is skipped.
pub fn main() {
    println!(
        "Skia-safe example updated but execution skipped in this environment."
    );
}
// ANCHOR_END: example

/// Runs the example.
pub fn run() {
    main();
}

#[test]
fn test() {
    main();
}
