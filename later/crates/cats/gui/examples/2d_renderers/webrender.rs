// ANCHOR: example
//! A simple WebRender example that draws a rotating rectangle.
use std::time::Instant;

use glutin::ContextBuilder;
use glutin::GlProfile;
use glutin::GlRequest;
use glutin::event::Event;
use glutin::event::WindowEvent;
use glutin::event_loop::ControlFlow;
use glutin::event_loop::EventLoop;
use glutin::window::WindowBuilder;
use webrender::api::units::DeviceIntSize;
use webrender::api::units::LayoutPoint;
use webrender::api::units::LayoutRect;
use webrender::api::units::LayoutSize;
use webrender::api::*;

fn main() {
    let event_loop = EventLoop::new();
    let window_builder = WindowBuilder::new().with_title("WebRender Example");
    let context_builder = ContextBuilder::new()
        .with_gl_profile(GlProfile::Core)
        .with_gl(GlRequest::Latest)
        .with_vsync(true);

    let windowed_context = context_builder
        .build_windowed(window_builder, &event_loop)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let gl =
        gl::Gl::load_with(|s| windowed_context.get_proc_address(s) as *const _);

    let mut size = windowed_context.window().inner_size();
    let mut width = size.width as i32;
    let mut height = size.height as i32;

    let builder = RendererBuilder::new().set_profiler(Box::new(NullProfiler));
    let mut renderer = builder
        .build(gl, DeviceIntSize::new(width, height))
        .unwrap();

    let document_id = renderer.create_document(DocumentId(0));
    let mut epoch = Epoch(0);
    let start_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    width = physical_size.width as i32;
                    height = physical_size.height as i32;
                    renderer.resize(DeviceIntSize::new(width, height));
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }
                _ => (),
            },
            Event::RedrawRequested(_) => {
                epoch.0 += 1;
                let current_time =
                    Instant::now().duration_since(start_time).as_secs_f32();

                let mut txn = Transaction::new();
                let mut display_list_builder = DisplayListBuilder::new(
                    LayoutSize::new(width as f32, height as f32),
                );

                let rect_size = LayoutSize::new(100.0, 50.0);
                let center =
                    LayoutPoint::new(width as f32 / 2.0, height as f32 / 2.0);
                display_list_builder.push_rect(
                    &LayoutRect::new(center - rect_size / 2.0, rect_size),
                    &SolidColor::new(ColorF::new(1.0, 0.0, 0.0, 1.0)),
                    &BorderRadius::zero(),
                    Transform::rotate(current_time),
                    None,
                );

                txn.set_display_list(
                    document_id,
                    LayoutSize::new(width as f32, height as f32),
                    display_list_builder.finalize(),
                );
                txn.generate_frame();

                renderer.update(txn);
                let mut frame_builder =
                    FrameBuilder::new(DeviceIntSize::new(width, height));
                frame_builder.begin(DeviceIntSize::new(width, height));
                renderer.render(epoch, frame_builder);
                windowed_context.swap_buffers().unwrap();
                windowed_context.window().request_redraw();
            }
            _ => (),
        }
    });
}

struct NullProfiler;

impl Profiler for NullProfiler {
    fn begin_section(&mut self, _a: &'static str) {}

    fn end_section(&mut self) {}
}
// ANCHOR_END: example

pub fn run() {
    main();
}
