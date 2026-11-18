// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Loads a mesh and presents it with three-d's window and model APIs.

use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use three_d::Camera;
use three_d::ClearState;
use three_d::ColorMaterial;
use three_d::CpuModel;
use three_d::FrameOutput;
use three_d::Model;
use three_d::Srgba;
use three_d::Window;
use three_d::WindowSettings;
use three_d::degrees;
use three_d::vec3;

/// Decodes an OBJ before any window or GPU surface is created.
fn load_scene(path: &Path) -> Result<CpuModel> {
    let mut assets = three_d_asset::io::load(&[path])
        .with_context(|| format!("read scene {}", path.display()))?;
    assets
        .deserialize("")
        .with_context(|| format!("decode scene {}", path.display()))
}

/// Uses one uploaded model and camera until the user closes the window.
fn main() -> Result<()> {
    let path = std::env::args_os().nth(1).map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/scene.obj"),
        PathBuf::from,
    );
    let cpu_model = load_scene(&path)?;
    let window = Window::new(WindowSettings {
        title: "three-d scene".into(),
        initial_size: Some((640, 480)),
        ..Default::default()
    })?;
    let context = window.gl();
    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.5),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let mut model = Model::<ColorMaterial>::new(&context, &cpu_model)?;
    for object in model.iter_mut() {
        object.material.color = Srgba::RED;
    }

    window.render_loop(move |frame| {
        let viewport = frame.viewport;
        if viewport.width == 0 || viewport.height == 0 {
            return FrameOutput {
                swap_buffers: false,
                ..FrameOutput::default()
            };
        }
        camera.set_viewport(viewport);
        frame
            .screen()
            .clear(ClearState::color_and_depth(0.0, 0.0, 0.0, 1.0, 1.0))
            .render(&camera, &model, &[]);
        FrameOutput::default()
    });
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// The bundled asset decodes into one geometry without opening a display.
    #[test]
    fn loads_bundled_triangle() -> Result<()> {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/scene.obj");
        assert_eq!(load_scene(&path)?.geometries.len(), 1);
        Ok(())
    }

    /// Asset failures report the path before entering the window lifecycle.
    #[test]
    fn reports_missing_asset() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/absent-scene.obj");
        let error = load_scene(&path).expect_err("the missing fixture must fail");
        assert!(error.to_string().contains("absent-scene.obj"));
    }
}
