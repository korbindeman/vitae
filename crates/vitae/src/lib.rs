pub mod prelude;
mod window;

pub use vitae_core as core;
pub use vitae_render as render;

use vitae_core::ElementBuilder;
use window::VitaeApp;
use winit::event_loop::EventLoop;

pub struct App {
    event_loop: EventLoop<()>,
    vitae_app: VitaeApp<'static>,
}

impl App {
    pub fn new(root_element: ElementBuilder) -> Self {
        App {
            event_loop: EventLoop::new().unwrap(),
            vitae_app: VitaeApp::new(root_element),
        }
    }

    pub fn run(mut self) {
        let _ = self.event_loop.run_app(&mut self.vitae_app);
    }
}
