// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Runs a headless `wgpu` compute shader that doubles each storage-buffer
//! value.

use std::error::Error;

use pollster::block_on;
use wgpu::util::DeviceExt;

const COMPUTE_SHADER: &str = r"
@group(0) @binding(0)
var<storage, read_write> values: array<u32>;

@compute @workgroup_size(64)
fn double_values(@builtin(global_invocation_id) id: vec3<u32>) {
    if (id.x < arrayLength(&values)) {
        values[id.x] *= 2u;
    }
}
";

const INPUT_VALUES: [u32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
const WORKGROUP_SIZE: u32 = 64;

/// Runs the compute example.
fn main() -> Result<(), Box<dyn Error>> {
    block_on(run_compute())
}

/// Requests a headless device, doubles the storage-buffer values, and reads
/// them back.
#[expect(
    clippy::little_endian_bytes,
    reason = "WGSL u32 storage buffers use little-endian byte order"
)]
async fn run_compute() -> Result<(), Box<dyn Error>> {
    let mut instance_desc =
        wgpu::InstanceDescriptor::new_without_display_handle();
    instance_desc.backends = wgpu::Backends::PRIMARY;
    let instance = wgpu::Instance::new(instance_desc);

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
            ..Default::default()
        })
        .await?;

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            label: Some("headless compute example device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            ..Default::default()
        })
        .await?;

    let input_bytes = INPUT_VALUES
        .iter()
        .flat_map(|value| value.to_le_bytes())
        .collect::<Vec<_>>();
    let storage_buffer =
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("values to double"),
            contents: &input_bytes,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        });
    let buffer_size = u64::try_from(input_bytes.len())?;
    let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("compute results"),
        size: buffer_size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("double values shader"),
        source: wgpu::ShaderSource::Wgsl(COMPUTE_SHADER.into()),
    });
    let pipeline =
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("double values pipeline"),
            layout: None,
            module: &shader,
            entry_point: Some("double_values"),
            compilation_options: Default::default(),
            cache: None,
        });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("values bind group"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: storage_buffer.as_entire_binding(),
        }],
    });

    let value_count = u32::try_from(INPUT_VALUES.len())?;
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("double values commands"),
        });
    {
        let mut compute_pass =
            encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("double values pass"),
                timestamp_writes: None,
            });
        compute_pass.set_pipeline(&pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch_workgroups(
            value_count.div_ceil(WORKGROUP_SIZE),
            1,
            1,
        );
    }
    encoder.copy_buffer_to_buffer(
        &storage_buffer,
        0,
        &readback_buffer,
        0,
        buffer_size,
    );
    let _submission = queue.submit([encoder.finish()]);

    let result = read_u32_values(&device, &readback_buffer)?;
    let expected = INPUT_VALUES.map(|value| value * 2);
    if result != expected {
        return Err(std::io::Error::other(format!(
            "compute result {result:?} did not match expected values {expected:?}"
        ))
        .into());
    }
    println!("Doubled values: {result:?}");
    Ok(())
}
// ANCHOR_END: example

/// Maps a staging buffer and converts its little-endian words to `u32` values.
///
/// # Errors
///
/// Returns an error if mapping, device polling, callback delivery, or byte
/// conversion fails.
#[expect(
    clippy::little_endian_bytes,
    reason = "WGSL u32 storage buffers use little-endian byte order"
)]
fn read_u32_values(
    device: &wgpu::Device,
    buffer: &wgpu::Buffer,
) -> Result<Vec<u32>, Box<dyn Error>> {
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| {
        // The receiver remains alive through polling and receiving the callback below.
        sender
            .send(result)
            .expect("the readback receiver remains in scope until mapping completes");
    });
    device.poll(wgpu::PollType::wait_indefinitely())?;
    receiver.recv()??;

    let values = {
        let mapped = buffer.slice(..).get_mapped_range()?;
        let (words, remainder) =
            mapped.as_chunks::<{ std::mem::size_of::<u32>() }>();
        if !remainder.is_empty() {
            return Err(std::io::Error::other(
                "readback size is not a multiple of four",
            )
            .into());
        }
        words.iter().copied().map(u32::from_le_bytes).collect()
    };
    buffer.unmap();
    Ok(values)
}

/// Runs the example's asynchronous compute workflow on a blocking executor.
pub fn run() -> Result<(), Box<dyn Error>> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the embedded WGSL shader parses and validates without a GPU.
    #[test]
    fn test_compute_shader_is_valid_wgsl() {
        let module = naga::front::wgsl::parse_str(COMPUTE_SHADER)
            .expect("the example shader should parse as WGSL");
        let mut validator = naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        );
        validator
            .validate(&module)
            .expect("the example shader should pass Naga validation");
    }

    /// Verifies the headless compute pipeline doubles each storage-buffer
    /// value.
    #[test]
    #[ignore = "requires a compatible GPU adapter"]
    fn test_compute_doubles_storage_values() -> Result<(), Box<dyn Error>> {
        run()
    }

    /// Exercises the complete compute workflow on an available GPU adapter.
    #[test]
    #[ignore = "requires a compatible GPU adapter"]
    fn test_main_entry_point() -> Result<(), Box<dyn Error>> {
        main()
    }
}
