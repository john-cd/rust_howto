#![allow(dead_code)]
// ANCHOR: example
use std::sync::Arc;
use vello::kurbo::{Affine, Circle, Rect, Stroke};
use vello::peniko::Color;
use vello::util::{RenderContext, RenderSurface};
use vello::{AaConfig, AaSupport, RenderParams, Renderer, RendererOptions, Scene};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::Window;

struct VelloApp {
    context: RenderContext,
    renderers: Vec<Option<Renderer>>,
    state: RenderState,
    scene: Scene,
}

enum RenderState {
    Active {
        surface: Box<RenderSurface<'static>>,
        window: Arc<Window>,
    },
    Suspended(Option<Arc<Window>>),
}

impl ApplicationHandler for VelloApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let RenderState::Suspended(cached) = &mut self.state else {
            return;
        };

        let window = cached.take().unwrap_or_else(|| {
            Arc::new(
                event_loop
                    .create_window(Window::default_attributes().with_title("Vello Example"))
                    .unwrap(),
            )
        });

        let size = window.inner_size();
        let surface = pollster::block_on(self.context.create_surface(
            window.clone(),
            size.width,
            size.height,
            vello::wgpu::PresentMode::AutoVsync,
        ))
        .expect("Failed to create surface");

        self.renderers
            .resize_with(self.context.devices.len(), || None);
        self.renderers[surface.dev_id].get_or_insert_with(|| {
            Renderer::new(
                &self.context.devices[surface.dev_id].device,
                RendererOptions {
                    antialiasing_support: AaSupport::all(),
                    ..Default::default()
                },
            )
            .expect("Failed to create renderer")
        });

        self.state = RenderState::Active {
            surface: Box::new(surface),
            window,
        };
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let RenderState::Active { surface, window } = &mut self.state else {
            return;
        };
        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => _event_loop.exit(),
            WindowEvent::RedrawRequested => {
                self.scene.reset();

                // Draw a red rectangle
                let rect = Rect::new(50.0, 50.0, 250.0, 150.0);
                self.scene.fill(
                    vello::peniko::Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgb8(255, 0, 0),
                    None,
                    &rect,
                );
                self.scene.stroke(
                    &Stroke::new(5.0),
                    Affine::IDENTITY,
                    Color::from_rgb8(0, 0, 255),
                    None,
                    &rect,
                );

                // Draw a green circle
                let circle = Circle::new((350.0, 100.0), 50.0);
                self.scene.fill(
                    vello::peniko::Fill::NonZero,
                    Affine::IDENTITY,
                    Color::from_rgb8(0, 255, 0),
                    None,
                    &circle,
                );

                let device = &self.context.devices[surface.dev_id];
                let surface_texture = surface.surface.get_current_texture().unwrap();
                let texture_view = surface_texture.texture.create_view(&vello::wgpu::TextureViewDescriptor::default());

                self.renderers[surface.dev_id]
                    .as_mut()
                    .unwrap()
                    .render_to_texture(
                        &device.device,
                        &device.queue,
                        &self.scene,
                        &texture_view,
                        &RenderParams {
                            base_color: Color::WHITE,
                            width: surface.config.width,
                            height: surface.config.height,
                            antialiasing_method: AaConfig::Msaa16,
                        },
                    )
                    .expect("Render failed");
                surface_texture.present();
            }
            WindowEvent::Resized(size) if size.width > 0 && size.height > 0 => {
                self.context
                    .resize_surface(surface, size.width, size.height);
                window.request_redraw();
            }
            _ => {}
        }
    }

    fn suspended(&mut self, _: &ActiveEventLoop) {
        if let RenderState::Active { window, .. } = &self.state {
            self.state = RenderState::Suspended(Some(window.clone()));
        }
    }
}

pub fn main() {
    let mut app = VelloApp {
        context: RenderContext::new(),
        renderers: vec![],
        state: RenderState::Suspended(None),
        scene: Scene::new(),
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut app).unwrap();
}
// ANCHOR_END: example
