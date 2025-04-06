// // ANCHOR: example
// // COMING SOON
// // ANCHOR_END: example
// //! This is a basic example of using `wgpu` to render a triangle.

// use wgpu::util::DeviceExt;
// use winit::event::*;
// use winit::event_loop::ControlFlow;
// use winit::event_loop::EventLoop;
// use winit::window::WindowBuilder;

// async fn run() {
//     env_logger::init();
//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new().build(&event_loop).unwrap();

//     let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
//     let surface = unsafe { instance.create_surface(&window) };
//     let adapter = instance
//         .request_adapter(&wgpu::RequestAdapterOptions {
//             power_preference: wgpu::PowerPreference::default(),
//             compatible_surface: Some(&surface),
//             force_fallback_adapter: false,
//         })
//         .await
//         .unwrap();
//     let (device, queue) = adapter
//         .request_device(
//             &wgpu::DeviceDescriptor {
//                 features: wgpu::Features::empty(),
//                 limits: wgpu::Limits::default(),
//                 label: None,
//             },
//             None, // Trace path
//         )
//         .await
//         .unwrap();

//     let swapchain_format = surface.get_preferred_format(&adapter).unwrap();
//     let size = window.inner_size();
//     let mut config = wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         format: swapchain_format,
//         width: size.width,
//         height: size.height,
//         present_mode: wgpu::PresentMode::Fifo,
//     };
//     surface.configure(&device, &config);

//     let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
//         label: Some("Shader"),
//         source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
//     });

//     let pipeline_layout =
//         device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("Pipeline Layout"),
//             bind_group_layouts: &[],
//             push_constant_ranges: &[],
//         });

//     let render_pipeline =
//         device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: Some("Render Pipeline"),
//             layout: Some(&pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &shader,
//                 entry_point: "vs_main",
//                 buffers: &[],
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &shader,
//                 entry_point: "fs_main",
//                 targets: &[wgpu::ColorTargetState {
//                     format: swapchain_format,
//                     blend: Some(wgpu::BlendState::REPLACE),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 }],
//             }),
//             primitive: wgpu::PrimitiveState::default(),
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState::default(),
//             multiview: None,
//         });

//     event_loop.run(move |event, _, control_flow| match event {
//         Event::WindowEvent {
//             ref event,
//             window_id,
//         } if window_id == window.id() => match event {
//             WindowEvent::CloseRequested
//             | WindowEvent::KeyboardInput {
//                 input:
//                     KeyboardInput {
//                         state: ElementState::Pressed,
//                         virtual_keycode: Some(VirtualKeyCode::Escape),
//                         ..
//                     },
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             WindowEvent::Resized(new_size) => {
//                 config.width = new_size.width;
//                 config.height = new_size.height;
//                 surface.configure(&device, &config);
//             }
//             WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                 config.width = new_inner_size.width;
//                 config.height = new_inner_size.height;
//                 surface.configure(&device, &config);
//             }
//             _ => {}
//         },
//         Event::RedrawRequested(_) => {
//             let frame = surface.get_current_texture().unwrap();
//             let view = frame
//                 .texture
//                 .create_view(&wgpu::TextureViewDescriptor::default());
//             let mut encoder = device.create_command_encoder(
//                 &wgpu::CommandEncoderDescriptor { label: None },
//             );

//             {
//                 let mut render_pass =
//                     encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                         label: Some("Render Pass"),
//                         color_attachments: &[wgpu::RenderPassColorAttachment {
//                             view: &view,
//                             resolve_target: None,
//                             ops: wgpu::Operations {
//                                 load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
//                                 store: true,
//                             },
//                         }],
//                         depth_stencil_attachment: None,
//                     });
//                 render_pass.set_pipeline(&render_pipeline);
//                 render_pass.draw(0..3, 0..1);
//             }

//             queue.submit(Some(encoder.finish()));
//             frame.present();
//         }
//         _ => {}
//     });
// }

// fn main() {
//     pollster::block_on(run());
// }

// #[test]
// fn test() {
//     main();
// }
// // [finish](https://github.com/john-cd/rust_howto/issues/772)
