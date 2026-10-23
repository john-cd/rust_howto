#![allow(dead_code)]
// TODO Reviewed ??-??-2026

use std::num::NonZeroU32;

use anyhow::Result;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::ContextAttributesBuilder;
use glutin::context::PossiblyCurrentContext;
use glutin::display::GetGlDisplay;
use glutin::prelude::*;
use glutin::surface::Surface;
use glutin::surface::SurfaceAttributesBuilder;
use glutin::surface::WindowSurface;
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

/// Creates a nonzero surface dimension, clamping minimized windows to one
/// pixel.
fn nonzero_dimension(dimension: u32) -> NonZeroU32 {
    NonZeroU32::new(dimension.max(1)).expect("a positive dimension is nonzero")
}

// ANCHOR: example
use femtovg::Canvas;
use femtovg::Color;
use femtovg::Paint;
use femtovg::Path;
use femtovg::renderer::OpenGl;

/// Returns the vertices of the example's four-sided path.
fn triangle_points() -> [(f32, f32); 4] {
    [(70.0, 70.0), (270.0, 70.0), (230.0, 210.0), (110.0, 210.0)]
}

/// Draws one colorful vector shape into a FemtoVG canvas.
fn draw_frame(
    canvas: &mut Canvas<OpenGl>,
    width: u32,
    height: u32,
    scale: f32,
) {
    canvas.set_size(width, height, scale);
    canvas.clear_rect(0, 0, width, height, Color::rgb(28, 32, 42));

    let points = triangle_points();
    let mut path = Path::new();
    path.move_to(points[0].0, points[0].1);
    for &(x, y) in &points[1..] {
        path.line_to(x, y);
    }
    path.close();
    canvas.fill_path(&path, &Paint::color(Color::rgb(55, 190, 150)));
    canvas.flush();
}
// ANCHOR_END: example

struct FemtoVgApp {
    window: Window,
    surface: Surface<WindowSurface>,
    context: PossiblyCurrentContext,
    canvas: Canvas<OpenGl>,
}

impl ApplicationHandler for FemtoVgApp {
    /// No additional resources are needed when the event loop resumes.
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    /// Handles resize, redraw, and close events for the renderer.
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if window_id != self.window.id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.surface.resize(
                    &self.context,
                    nonzero_dimension(size.width),
                    nonzero_dimension(size.height),
                );
                self.window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                let size = self.window.inner_size();
                draw_frame(
                    &mut self.canvas,
                    size.width,
                    size.height,
                    self.window.scale_factor() as f32,
                );
                self.surface
                    .swap_buffers(&self.context)
                    .expect("the OpenGL window swaps its rendered frame");
            }
            _ => {}
        }
    }

    /// Requests another redraw when the event loop becomes idle.
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }
}

/// Opens an OpenGL window and renders a simple FemtoVG path until it closes.
pub fn run() -> Result<()> {
    let event_loop = EventLoop::new()?;
    let (window, config) = DisplayBuilder::new()
        .with_window_attributes(Some(
            Window::default_attributes().with_title("FemtoVG path"),
        ))
        .build(&event_loop, ConfigTemplateBuilder::new(), |configs| {
            configs
                .reduce(|best, candidate| {
                    if candidate.num_samples() > best.num_samples() {
                        candidate
                    } else {
                        best
                    }
                })
                .expect("the system provides an OpenGL configuration")
        })
        .map_err(|error| {
            anyhow::anyhow!("could not create the OpenGL window: {error}")
        })?;
    let window =
        window.expect("the display builder creates the requested window");
    let window_handle = window.window_handle()?.as_raw();
    let display = config.display();
    let context_attributes =
        ContextAttributesBuilder::new().build(Some(window_handle));

    // Safety: the window handle remains alive for the lifetime of the GL
    // context.
    let not_current_context =
        unsafe { display.create_context(&config, &context_attributes)? };
    let size = window.inner_size();
    let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
        .build(
            window_handle,
            nonzero_dimension(size.width),
            nonzero_dimension(size.height),
        );

    // Safety: the surface uses the live window handle and matching display
    // configuration.
    let surface =
        unsafe { display.create_window_surface(&config, &surface_attributes)? };
    let context = not_current_context.make_current(&surface)?;
    // Safety: the GL context is current and each function pointer comes from
    // its display.
    let renderer = unsafe {
        OpenGl::new_from_function_cstr(|name| display.get_proc_address(name))?
    };
    let canvas = Canvas::new(renderer)?;
    let mut app = FemtoVgApp {
        window,
        surface,
        context,
        canvas,
    };
    event_loop.set_control_flow(ControlFlow::Wait);
    event_loop.run_app(&mut app)?;
    Ok(())
}

/// Starts the interactive window example as a Cargo example target.
fn main() -> Result<()> {
    run()
}

/// The interactive renderer requires a desktop display and OpenGL context.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies the pure path data before it is sent to the GPU renderer.
    #[test]
    fn triangle_uses_four_expected_vertices() {
        assert_eq!(
            triangle_points(),
            [(70.0, 70.0), (270.0, 70.0), (230.0, 210.0), (110.0, 210.0)]
        );
    }

    /// Opens the window when a developer explicitly runs the ignored GUI test.
    #[test]
    #[ignore = "requires an interactive desktop display and OpenGL context"]
    fn test_main() -> anyhow::Result<()> {
        super::main()
    }
}
