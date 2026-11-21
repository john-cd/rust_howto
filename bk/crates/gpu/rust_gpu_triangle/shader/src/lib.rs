// TODO Reviewed ??-??-2026.
//! Writes the vertices and flat color of a red triangle.

#![no_std]
#![allow(
    unexpected_cfgs,
    reason = "Rust-GPU defines the SPIR-V target architecture"
)]

use spirv_std::glam::Vec4;
use spirv_std::glam::vec4;
use spirv_std::spirv;

/// Writes the clip-space position for one of the three vertices.
#[spirv(vertex)]
pub fn main_vs(#[spirv(vertex_index)] vertex_index: u32, #[spirv(position)] position: &mut Vec4) {
    *position = match vertex_index {
        0 => vec4(-0.75, -0.75, 0.0, 1.0),
        1 => vec4(0.75, -0.75, 0.0, 1.0),
        _ => vec4(0.0, 0.75, 0.0, 1.0),
    };
}

/// Writes opaque red to the color attachment.
#[spirv(fragment)]
pub fn main_fs(#[spirv(location = 0)] color: &mut Vec4) {
    *color = vec4(1.0, 0.0, 0.0, 1.0);
}
