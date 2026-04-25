#![allow(dead_code)]
// ANCHOR: example
#[cfg(target_arch = "wasm32")]
use spirv_std::glam::{vec4, Vec4};
#[cfg(target_arch = "wasm32")]
use spirv_std::spirv;

#[cfg(target_arch = "wasm32")]
#[spirv(fragment)]
pub fn main_fs(output: &mut Vec4) {
    *output = vec4(1.0, 0.0, 0.0, 1.0);
}

#[cfg(target_arch = "wasm32")]
#[spirv(vertex)]
pub fn main_vs(
    #[spirv(vertex_index)] vert_id: i32,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    *out_pos = vec4(
        (vert_id - 1) as f32,
        ((vert_id & 1) * 2 - 1) as f32,
        0.0,
        1.0,
    );
}
// ANCHOR_END: example

fn main() {}

#[test]
fn test() {
    main();
}
