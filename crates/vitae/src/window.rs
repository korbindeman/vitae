use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use vitae_core::ElementBuilder;
use vitae_render::Renderer;

pub struct VitaeApp<'a> {
    renderer: Option<Renderer<'a>>,
    root_element: ElementBuilder,
}

impl<'a> VitaeApp<'a> {
    pub fn new(root_element: ElementBuilder) -> Self {
        Self {
            renderer: None,
            root_element,
        }
    }
}

impl<'a> ApplicationHandler for VitaeApp<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop
            .create_window(Window::default_attributes().with_title("vitae"))
            .unwrap();
        self.renderer = Some(Renderer::new(window, self.root_element.clone()));
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

        if renderer.window().id() == window_id {
            match event {
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::Resized(physical_size) => {
                    renderer.resize(physical_size);
                }
                WindowEvent::RedrawRequested => {
                    renderer.render().unwrap();
                }
                _ => {}
            }
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_ref() {
            renderer.window().request_redraw();
        }
    }
}
