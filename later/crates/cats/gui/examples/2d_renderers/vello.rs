// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Renders a simple 2D scene into a Vello GPU texture.

use std::num::NonZeroUsize;

use anyhow::Result;
use pollster::block_on;
use vello::AaConfig;
use vello::Renderer;
use vello::RendererOptions;
use vello::Scene;
use vello::kurbo::Affine;
use vello::kurbo::Circle;
use vello::kurbo::Rect;
use vello::peniko::Color;
use vello::util::RenderContext;
use wgpu::Extent3d;
use wgpu::Texture;
use wgpu::TextureDescriptor;
use wgpu::TextureDimension;
use wgpu::TextureFormat;
use wgpu::TextureUsages;
use wgpu::TextureView;

/// Creates a new Vello renderer using the provided WGPU device.
fn create_vello_renderer(device: &wgpu::Device) -> Result<Renderer> {
    Ok(Renderer::new(
        device,
        RendererOptions {
            use_cpu: false,
            antialiasing_support: vello::AaSupport::all(),
            num_init_threads: NonZeroUsize::new(1),
            ..Default::default()
        },
    )?)
}

/// Creates an off-screen render target texture and its texture view.
fn create_target_texture(
    device: &wgpu::Device,
    width: u32,
    height: u32,
) -> (Texture, TextureView) {
    let target = device.create_texture(&TextureDescriptor {
        label: Some("Vello example target"),
        size: Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba8Unorm,
        usage: TextureUsages::STORAGE_BINDING | TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let target_view =
        target.create_view(&wgpu::TextureViewDescriptor::default());
    (target, target_view)
}

/// Creates a WGPU device and renders the example scene into a texture.
async fn render_scene() -> Result<()> {
    let mut context = RenderContext::new();
    let device_id = context.device(None).await.ok_or_else(|| {
        anyhow::anyhow!("No compatible GPU adapter was found")
    })?;
    let device_handle = &context.devices[device_id];
    let device = &device_handle.device;
    let queue = &device_handle.queue;
    let mut renderer = create_vello_renderer(device)?;

    let (width, height) = (800, 600);
    let (_target, target_view) = create_target_texture(device, width, height);
    let scene = build_scene();

    renderer.render_to_texture(
        device,
        queue,
        &scene,
        &target_view,
        &vello::RenderParams {
            base_color: Color::WHITE,
            width,
            height,
            antialiasing_method: AaConfig::Msaa8,
        },
    )?;

    Ok(())
}

/// Renders a scene into an off-screen texture with Vello.
pub fn run() -> Result<()> {
    block_on(render_scene())
}

/// Renders a scene into an off-screen texture with Vello.
fn main() -> Result<()> {
    run()
}

/// Builds a scene containing a filled rectangle, an outlined rectangle, and a
/// circle.
fn build_scene() -> Scene {
    let mut scene = Scene::new();
    let rectangle = Rect::new(50.0, 50.0, 250.0, 150.0);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        Color::from_rgb8(220, 70, 70),
        None,
        &rectangle,
    );
    scene.stroke(
        &vello::kurbo::Stroke::new(5.0),
        Affine::IDENTITY,
        Color::from_rgb8(40, 80, 220),
        None,
        &rectangle,
    );

    let circle = Circle::new((400.0, 120.0), 60.0);
    scene.fill(
        vello::peniko::Fill::NonZero,
        Affine::IDENTITY,
        Color::from_rgb8(60, 180, 90),
        None,
        &circle,
    );
    scene
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms scene-building encodes drawing commands without creating a GPU.
    #[test]
    fn scene_building_encodes_shapes_without_a_gpu() {
        let scene = build_scene();
        assert!(!scene.encoding().is_empty());
    }

    /// Renders the Vello example when a compatible GPU adapter is available.
    #[test]
    #[ignore = "requires a compatible GPU adapter"]
    fn vello_renderer_entry_point_renders_with_a_gpu() -> Result<()> {
        main()
    }
}
