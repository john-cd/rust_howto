// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Exposes a button in a native Winit window through AccessKit.
//!
//! The tree describes the window and its labeled button in semantic terms, so
//! assistive technology can discover the control without relying on its visual
//! appearance. `Adapter` connects that tree to Winit: window events are passed
//! to it, and an initial-tree request causes the current tree to be supplied.
//! Close the window to end the event loop. Run this example on a desktop with a
//! display and an accessibility bridge supported by the platform.

use accesskit::Node;
use accesskit::NodeId;
use accesskit::Role;
use accesskit::TreeId;
use accesskit::TreeInfo;
use accesskit::TreeUpdate;
use accesskit_winit::Adapter;
use accesskit_winit::Event as AccessEvent;
use accesskit_winit::WindowEvent as AccessWindowEvent;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::ControlFlow;
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::window::WindowId;

/// The semantic interface seen by a screen reader.
fn tree() -> TreeUpdate {
    let root = NodeId(0);
    let button = NodeId(1);
    let mut window = Node::new(Role::Window);
    window.set_children(vec![button]);
    let mut save = Node::new(Role::Button);
    save.set_label("Save document");
    TreeUpdate {
        nodes: vec![(root, window), (button, save)],
        tree: Some(TreeInfo::new(root)),
        tree_id: TreeId::ROOT,
        focus: button,
    }
}

/// Routes native window events to the accessibility adapter.
struct App {
    window: Option<Window>,
    adapter: Option<Adapter>,
    proxy: winit::event_loop::EventLoopProxy<AccessEvent>,
}

impl ApplicationHandler<AccessEvent> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attributes = Window::default_attributes()
            .with_title("Accessible save button")
            .with_visible(false);
        match event_loop.create_window(attributes) {
            Ok(window) => {
                let mut adapter =
                    Adapter::with_event_loop_proxy(event_loop, &window, self.proxy.clone());
                adapter.update_if_active(tree);
                window.set_visible(true);
                self.adapter = Some(adapter);
                self.window = Some(window);
            }
            Err(error) => {
                eprintln!("Could not create the window: {error}");
                event_loop.exit();
            }
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        if let (Some(window), Some(adapter)) = (&self.window, &mut self.adapter)
            && window.id() == id
        {
            adapter.process_event(window, &event);
            if matches!(event, WindowEvent::CloseRequested) {
                event_loop.exit();
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: AccessEvent) {
        if let Some(adapter) = &mut self.adapter
            && matches!(event.window_event, AccessWindowEvent::InitialTreeRequested)
        {
            adapter.update_if_active(tree);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::<AccessEvent>::with_user_event().build()?;
    event_loop.set_control_flow(ControlFlow::Wait);
    let proxy = event_loop.create_proxy();
    event_loop.run_app(&mut App {
        window: None,
        adapter: None,
        proxy,
    })?;
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_exposes_focused_button() {
        let update = tree();
        assert_eq!(update.focus, NodeId(1));
        assert_eq!(update.nodes[1].1.role(), Role::Button);
        assert_eq!(update.nodes[1].1.label(), Some("Save document"));
    }
}
