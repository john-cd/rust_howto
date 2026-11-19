// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Renders a Rust-GPU triangle to a texture and checks the readback.

use std::sync::mpsc;
use std::time::Duration;

use anyhow::Result;
use anyhow::ensure;

/// A 64-pixel RGBA row is exactly the required 256-byte copy alignment.
const SIZE: u32 = 64;

/// Validated Rust-GPU SPIR-V embedded by the host build script.
const SHADER: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/triangle.spv"));

/// Renders one frame with a native adapter and copies tightly packed RGBA bytes.
///
/// # Errors
///
/// Returns an error if adapter/device selection, GPU completion, or mapping fails.
async fn render() -> Result<Vec<u8>> {
    let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
    descriptor.backends = wgpu::Backends::VULKAN;
    let instance = wgpu::Instance::new(descriptor);
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions::default())
        .await?;
    println!("Adapter: {:?}", adapter.get_info());
    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default())
        .await?;
    let size = wgpu::Extent3d {
        width: SIZE,
        height: SIZE,
        depth_or_array_layers: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("offscreen triangle"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("RGBA readback"),
        size: u64::from(SIZE * SIZE * 4),
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("triangle shader"),
        source: wgpu::util::make_spirv(SHADER),
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("triangle pipeline"),
        layout: None,
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("main_vs"),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: Default::default(),
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("main_fs"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: wgpu::TextureFormat::Rgba8Unorm,
                blend: None,
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask: None,
        cache: None,
    });
    let view = texture.create_view(&Default::default());
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("triangle pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
                    store: wgpu::StoreOp::Store,
                },
            })],
            ..Default::default()
        });
        pass.set_pipeline(&pipeline);
        pass.draw(0..3, 0..1);
    }
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &readback,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(SIZE * 4),
                rows_per_image: Some(SIZE),
            },
        },
        size,
    );
    let submission = queue.submit([encoder.finish()]);
    let (sender, receiver) = mpsc::sync_channel(1);
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            // A timed-out caller can drop the receiver before the callback.
            let _ = sender.send(result);
        });
    device.poll(wgpu::PollType::Wait {
        submission_index: Some(submission),
        timeout: Some(Duration::from_secs(10)),
    })?;
    receiver.recv_timeout(Duration::from_secs(10))??;
    let bytes = readback.slice(..).get_mapped_range()?.to_vec();
    readback.unmap();
    Ok(bytes)
}

/// Checks an interior triangle pixel and an exterior clear-color pixel.
///
/// # Errors
///
/// Returns an error for a truncated frame, a missing triangle, or a wrong clear color.
fn verify_frame(bytes: &[u8]) -> Result<()> {
    ensure!(
        bytes.len() == (SIZE * SIZE * 4) as usize,
        "unexpected frame length"
    );
    let center = ((SIZE / 2 * SIZE + SIZE / 2) * 4) as usize;
    ensure!(
        bytes[center..center + 4] == [255, 0, 0, 255],
        "triangle center is not red"
    );
    ensure!(bytes[..4] == [0, 0, 255, 255], "frame corner is not blue");
    Ok(())
}

/// Draws and validates one frame, then exits without opening a window.
///
/// # Errors
///
/// Returns an error if the native GPU workflow or pixel validation fails.
fn main() -> Result<()> {
    let bytes = pollster::block_on(render())?;
    verify_frame(&bytes)?;
    println!("Verified a red triangle on a blue 64x64 RGBA frame.");
    Ok(())
}
// ANCHOR_END: example

/// Tests frame verification and the optional native render.
#[cfg(test)]
mod tests {
    use super::*;

    /// Rejects truncated or all-clear output instead of reporting render success.
    #[test]
    fn rejects_incomplete_or_missing_triangle() {
        assert!(verify_frame(&[]).is_err());
        let clear = [0, 0, 255, 255].repeat((SIZE * SIZE) as usize);
        assert!(verify_frame(&clear).is_err());
    }

    /// Draws and verifies pixels on a real native adapter when explicitly requested.
    #[test]
    #[ignore = "requires a native GPU adapter and installed driver"]
    fn renders_triangle_on_native_gpu() -> Result<()> {
        main()
    }
}
