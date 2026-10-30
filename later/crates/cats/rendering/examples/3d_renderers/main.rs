//! Projects a 3D triangle to screen space the way a renderer's vertex stage
//! does.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: example
use glam::Mat4;
use glam::Vec2;
use glam::Vec3;
use glam::Vec4;
use glam::camera::rh::proj::directx;
use glam::camera::rh::view;

const WIDTH: f32 = 640.0;
const HEIGHT: f32 = 480.0;

/// Builds the model-view-projection matrix for a camera looking at the origin.
///
/// The projection uses the right-handed Y-up convention with a depth range of
/// `0.0..=1.0`, as in `DirectX`, Vulkan, Metal, and WebGPU.
fn model_view_projection() -> Mat4 {
    let model = Mat4::from_rotation_y(core::f32::consts::FRAC_PI_4);
    let view =
        view::look_at_mat4(Vec3::new(0.0, 0.0, 3.0), Vec3::ZERO, Vec3::Y);
    let projection = directx::perspective(
        core::f32::consts::FRAC_PI_3,
        WIDTH / HEIGHT,
        0.1,
        100.0,
    );
    projection * view * model
}

/// Transforms a model-space position into pixel coordinates (origin at the
/// top left) and a depth in `0.0..=1.0`, or `None` when it is clipped.
fn project(mvp: Mat4, position: Vec3) -> Option<(Vec2, f32)> {
    let clip: Vec4 = mvp * position.extend(1.0);
    if clip.w <= 0.0 {
        return None; // Behind the camera.
    }
    let ndc = clip.truncate() / clip.w; // Perspective divide.
    if ndc.x.abs() > 1.0 || ndc.y.abs() > 1.0 || !(0.0..=1.0).contains(&ndc.z) {
        return None; // Outside the view volume.
    }
    let pixel = Vec2::new(
        (ndc.x * 0.5 + 0.5) * WIDTH,
        (1.0 - (ndc.y * 0.5 + 0.5)) * HEIGHT,
    );
    Some((pixel, ndc.z))
}

/// Projects a triangle and prints its screen-space vertices.
fn main() {
    let mvp = model_view_projection();
    let triangle = [
        Vec3::new(-1.0, -1.0, 0.0),
        Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    for vertex in triangle {
        match project(mvp, vertex) {
            Some((pixel, depth)) => {
                println!("{vertex} -> pixel {pixel}, depth {depth:.3}");
            }
            None => println!("{vertex} is clipped"),
        }
    }
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the example through its entry point.
    #[test]
    fn test_main_entry_point() {
        main();
    }

    /// Verifies the origin lands at the screen center in front of the camera.
    #[test]
    fn projects_the_origin_to_the_screen_center() {
        let (pixel, depth) = project(model_view_projection(), Vec3::ZERO)
            .expect("the origin is visible");
        assert!((pixel - Vec2::new(WIDTH / 2.0, HEIGHT / 2.0)).length() < 1e-3);
        assert!((0.0..1.0).contains(&depth));
    }

    /// Verifies points behind the camera are clipped.
    #[test]
    fn clips_points_behind_the_camera() {
        assert!(
            project(model_view_projection(), Vec3::new(0.0, 0.0, 10.0))
                .is_none()
        );
    }
}
