//! Uploads a grayscale image, runs both compute passes, and reads the result back.

use std::sync::mpsc;
use std::time::Duration;

use anyhow::Context as _;
use anyhow::Result;
use wgpu::util::DeviceExt as _;

const SHADER: &str = include_str!("blur.wgsl");
const GROUP_SIZE: u32 = 16;
const GPU_TIMEOUT: Duration = Duration::from_secs(30);

/// Holds the adapter's device, queue, and reusable blur pipelines.
pub struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    horizontal: wgpu::ComputePipeline,
    vertical: wgpu::ComputePipeline,
}

/// Holds reusable buffers and bindings for a square image.
pub struct Image {
    width: u32,
    input: wgpu::Buffer,
    output: wgpu::Buffer,
    readback: wgpu::Buffer,
    horizontal: wgpu::BindGroup,
    vertical: wgpu::BindGroup,
}

impl Gpu {
    /// Selects a native compute adapter and creates both pipelines.
    ///
    /// # Errors
    ///
    /// Returns an error if adapter or device creation fails.
    pub async fn new() -> Result<Self> {
        let mut descriptor = wgpu::InstanceDescriptor::new_without_display_handle();
        let backends = wgpu::Backends::PRIMARY.with_env();
        descriptor.backends = backends;
        let instance = wgpu::Instance::new(descriptor);
        let adapter = if let Ok(name) = std::env::var("WGPU_ADAPTER_NAME") {
            let filter = name.to_lowercase();
            instance
                .enumerate_adapters(backends)
                .await
                .into_iter()
                .find(|adapter| adapter.get_info().name.to_lowercase().contains(&filter))
                .with_context(|| format!("no adapter matched {name:?} on {backends:?}"))?
        } else {
            instance
                .request_adapter(&wgpu::RequestAdapterOptions::default())
                .await?
        };
        let info = adapter.get_info();
        println!(
            "Adapter: {} ({:?}, {:?})",
            info.name, info.backend, info.device_type
        );
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await?;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("blur.wgsl"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let pipeline = |entry| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: None,
                module: &module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let horizontal = pipeline("horizontal");
        let vertical = pipeline("vertical");
        Ok(Self {
            device,
            queue,
            horizontal,
            vertical,
        })
    }

    /// Allocates and binds resources for a benchmark image of the given width.
    pub fn prepare(&self, width: u32) -> Image {
        let size = u64::from(width) * u64::from(width) * 4;
        let buffer = |label, usage| {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size,
                usage,
                mapped_at_creation: false,
            })
        };
        let storage = wgpu::BufferUsages::STORAGE;
        let input = buffer("source", storage | wgpu::BufferUsages::COPY_DST);
        let scratch = buffer("horizontal result", storage);
        let output = buffer("blurred image", storage | wgpu::BufferUsages::COPY_SRC);
        let readback = buffer(
            "CPU readback",
            wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        );
        let dimensions = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("width, height, padding"),
                contents: bytemuck::cast_slice(&[width, width, 0, 0]),
                usage: wgpu::BufferUsages::UNIFORM,
            });

        let bindings =
            |pipeline: &wgpu::ComputePipeline, src: &wgpu::Buffer, dst: &wgpu::Buffer| {
                self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &pipeline.get_bind_group_layout(0),
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: dimensions.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: src.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: dst.as_entire_binding(),
                        },
                    ],
                })
            };
        let horizontal = bindings(&self.horizontal, &input, &scratch);
        let vertical = bindings(&self.vertical, &scratch, &output);
        Image {
            width,
            input,
            output,
            readback,
            horizontal,
            vertical,
        }
    }

    /// Runs both passes and copies the output into the caller's host buffer.
    ///
    /// The input and output slices must contain one `u32` per image pixel.
    ///
    /// # Errors
    ///
    /// Returns an error if submitting, waiting for, or mapping the output fails.
    pub fn blur(&self, image: &Image, input: &[u32], output: &mut [u32]) -> Result<()> {
        self.queue
            .write_buffer(&image.input, 0, bytemuck::cast_slice(input));
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode_passes(image, &mut encoder);
        encoder.copy_buffer_to_buffer(&image.output, 0, &image.readback, 0, image.output.size());
        let submission = self.queue.submit([encoder.finish()]);
        let (sender, receiver) = mpsc::sync_channel(1);
        image
            .readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                // The caller can time out and drop the receiver.
                let _ = sender.send(result);
            });
        self.device.poll(wgpu::PollType::Wait {
            submission_index: Some(submission),
            timeout: Some(GPU_TIMEOUT),
        })?;
        receiver.recv_timeout(GPU_TIMEOUT)??;
        {
            let mapped = image.readback.slice(..).get_mapped_range()?;
            bytemuck::cast_slice_mut(output).copy_from_slice(&mapped);
        }
        image.readback.unmap();
        Ok(())
    }

    /// Runs the same blur using the input uploaded by a prior call to `blur`.
    ///
    /// # Errors
    ///
    /// Returns an error if waiting for the submitted work fails.
    pub fn blur_resident(&self, image: &Image) -> Result<()> {
        let mut encoder = self.device.create_command_encoder(&Default::default());
        self.encode_passes(image, &mut encoder);
        let submission = self.queue.submit([encoder.finish()]);
        self.device.poll(wgpu::PollType::Wait {
            submission_index: Some(submission),
            timeout: Some(GPU_TIMEOUT),
        })?;
        Ok(())
    }

    /// Encodes separate passes so the vertical stage can consume the horizontal result.
    fn encode_passes(&self, image: &Image, encoder: &mut wgpu::CommandEncoder) {
        let groups = image.width.div_ceil(GROUP_SIZE);
        for (pipeline, bindings) in [
            (&self.horizontal, &image.horizontal),
            (&self.vertical, &image.vertical),
        ] {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, bindings, &[]);
            pass.dispatch_workgroups(groups, groups, 1);
        }
    }
}
