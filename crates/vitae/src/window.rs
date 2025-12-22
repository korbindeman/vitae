use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use vitae_core::ElementBuilder;
use vitae_render::Renderer;

use crate::signal::{reset_signal_counter, take_redraw_request};

pub struct VitaeApp<'a, M: Clone> {
    renderer: Option<Renderer<'a>>,
    model: M,
    view_fn: fn(&M) -> ElementBuilder,
    cursor_position: (f64, f64),
}

impl<'a, M: Clone + 'static> VitaeApp<'a, M> {
    pub fn new(initial_model: M, view: fn(&M) -> ElementBuilder) -> Self {
        Self {
            renderer: None,
            model: initial_model,
            view_fn: view,
            cursor_position: (0.0, 0.0),
        }
    }

    fn build_tree(&self) -> ElementBuilder {
        // Reset signal counter for consistent IDs across renders
        reset_signal_counter();
        (self.view_fn)(&self.model)
    }
}

impl<'a, M: Clone + 'static> ApplicationHandler for VitaeApp<'a, M> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("vitae"))
            .unwrap();
        let root = self.build_tree();
        self.renderer = Some(Renderer::new(window, root));
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        if renderer.window().id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                renderer.resize(physical_size);
            }
            WindowEvent::RedrawRequested => {
                // Build tree separately to avoid borrow issues
                let root = self.build_tree();
                // Then use renderer
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.set_root(root);
                    renderer.render().unwrap();
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor_position = (position.x, position.y);
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                // Perform hit-testing and call handler if found
                let (x, y) = self.cursor_position;
                let handler = renderer.hit_test(x as f32, y as f32);
                // handler is now owned, renderer borrow ends here

                if let Some(handler) = handler {
                    handler(&mut self.model);
                    // Trigger redraw after model update
                    if let Some(renderer) = self.renderer.as_ref() {
                        renderer.window().request_redraw();
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_ref() {
            // Check if any signal requested a redraw
            if take_redraw_request() {
                renderer.window().request_redraw();
            } else {
                // Still request redraw for continuous rendering (for now)
                // TODO: Make this opt-in for apps that need it
                renderer.window().request_redraw();
            }
        }
    }
}
