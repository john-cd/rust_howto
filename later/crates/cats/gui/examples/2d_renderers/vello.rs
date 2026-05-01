// ANCHOR: example
//! A minimal Vello example using `winit`, `wgpu`, and `vello`.
//!
//! It creates a window, builds a simple 2D scene, and renders it every frame.
use std::time::Instant;

use pollster::block_on;
use vello::AaConfig;
use vello::RenderParams;
use vello::Renderer;
use vello::RendererOptions;
use vello::SceneBuilder;
use vello::kurbo::Affine;
use vello::kurbo::Circle;
use vello::kurbo::Rect;
use vello::peniko::Brush;
use vello::peniko::Color;
use vello::peniko::Fill;
use vello::peniko::Stroke;
use winit::dpi::PhysicalSize;
use winit::event::Event;
use winit::event::WindowEvent;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Vello Example")
        .with_inner_size(PhysicalSize::new(800, 600))
        .build(&event_loop)
        .expect("Failed to create window");

    let instance = wgpu::Instance::new(wgpu::Backends::all());
    let surface = unsafe { instance.create_surface(&window) }
        .expect("Failed to create surface");

    let adapter =
        block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        }))
        .expect("Failed to request adapter");

    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::downlevel_defaults(),
        },
        None,
    ))
    .expect("Failed to request device");

    let mut surface_config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::STORAGE_BINDING,
        format: surface.get_supported_formats(&adapter)[0],
        width: 800,
        height: 600,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &surface_config);

    let mut renderer = Renderer::new(&device, RendererOptions::default())
        .expect("Failed to create Vello renderer");

    let start_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(new_size) => {
                    if new_size.width > 0 && new_size.height > 0 {
                        surface_config.width = new_size.width;
                        surface_config.height = new_size.height;
                        surface.configure(&device, &surface_config);
                    }
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let scene = build_scene();
                let render_params = RenderParams {
                    base_color: Color::WHITE,
                    width: surface_config.width,
                    height: surface_config.height,
                    antialiasing_method: AaConfig::Msaa8,
                };

                let surface_texture = match surface.get_current_texture() {
                    Ok(surface_texture) => surface_texture,
                    Err(err) => {
                        eprintln!("Failed to acquire surface texture: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                };

                let texture_view = surface_texture
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default());

                if let Err(err) = renderer.render_to_texture(
                    &device,
                    &queue,
                    &scene,
                    &texture_view,
                    &render_params,
                ) {
                    eprintln!("Failed to render Vello scene: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                surface_texture.present();
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

fn build_scene() -> vello::Scene {
    let mut scene_builder = SceneBuilder::new();

    let rect = Rect::new(50.0, 50.0, 250.0, 150.0);
    scene_builder.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        &Brush::Solid(Color::RED),
        None,
        &rect,
    );
    scene_builder.stroke(
        &Stroke::new(5.0),
        Affine::IDENTITY,
        &Brush::Solid(Color::BLUE),
        None,
        &rect,
    );

    let circle = Circle::new((400.0, 120.0), 60.0);
    scene_builder.fill(
        Fill::NonZero,
        Affine::IDENTITY,
        &Brush::Solid(Color::GREEN),
        None,
        &circle,
    );

    scene_builder.build()
}

// ANCHOR_END: example

pub fn run() {
    main();
}
