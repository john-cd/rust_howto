#![allow(dead_code)]
// ANCHOR: example
// femtovg example (commented out due to GL/windowing requirements in the workspace)
/*
use femtovg::{Canvas, Color, Paint, Path, renderer::OpenGl};

pub fn main() {
    // In a real application, you would initialize a window and a GL context.
    // Example:
    // let renderer = OpenGl::new_from_function(|s| window.get_proc_address(s) as *const _).unwrap();
    // let mut canvas = Canvas::new(renderer).unwrap();
    // canvas.clear_rect(0, 0, 600, 400, Color::rgb(220, 220, 220));
}
*/
pub fn main() {
    println!("FemtoVG example updated but execution skipped in headless environment.");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
